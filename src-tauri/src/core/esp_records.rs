//! Свой парсер .esp/.esm/.esl: считаем типы записей по сигнатурам.
//!
//! Не читаем содержимое записей — только заголовки. Это позволяет
//! посчитать, сколько записей каждого типа (WEAP, ARMO, NPC_ и т.д.)
//! в плагине, не зная схем подзаписей.
//!
//! Формат (Skyrim SE / Fallout 4):
//! - TES4 header: 24 байта
//! - Subrecords заголовка (HEDR, MAST, ...)
//! - GRUP: 24 байта заголовок, `size` включает заголовок
//! - Запись: 24 байта заголовок + N байт данных

use crate::models::RecordTypeCount;
use anyhow::{bail, Context, Result};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

/// Максимальный размер данных записи — защита от битых файлов.
const MAX_RECORD_SIZE: u32 = 64 * 1024 * 1024;

/// Максимальная глубина вложенности GRUP — защита от бесконечной рекурсии.
const MAX_GROUP_DEPTH: usize = 16;

/// Читает плагин и возвращает список типов записей с их количеством.
pub fn count_record_types(path: &Path) -> Result<Vec<RecordTypeCount>> {
    let file = File::open(path)
        .with_context(|| format!("Не удалось открыть: {}", path.display()))?;
    let file_size = file.metadata()?.len();
    let mut reader = BufReader::new(file);

    // 1. TES4-заголовок (24 байта).
    let mut header = [0u8; 24];
    reader
        .read_exact(&mut header)
        .context("Не удалось прочитать TES4 заголовок")?;

    if &header[0..4] != b"TES4" {
        bail!("Файл не начинается с сигнатуры TES4");
    }

    let header_data_size =
        u32::from_le_bytes([header[4], header[5], header[6], header[7]]);

    // 2. Пропускаем данные заголовка (HEDR, MAST, CNAM, ...).
    reader
        .seek(SeekFrom::Current(header_data_size as i64))
        .context("Не удалось пропустить данные TES4 заголовка")?;

    // 3. Считаем записи.
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    parse_top_level(&mut reader, file_size, &mut counts)?;

    Ok(counts
        .into_iter()
        .map(|(record_type, count)| RecordTypeCount { record_type, count })
        .collect())
}

/// Идёт по верхнему уровню файла: GRUP или записи.
fn parse_top_level<R: Read + Seek>(
    reader: &mut R,
    file_size: u64,
    counts: &mut BTreeMap<String, usize>,
) -> Result<()> {
    loop {
        let pos = reader.stream_position()?;
        if file_size.saturating_sub(pos) < 24 {
            break;
        }

        let mut sig = [0u8; 4];
        if reader.read_exact(&mut sig).is_err() {
            break;
        }

        let mut size_buf = [0u8; 4];
        reader.read_exact(&mut size_buf)?;
        let size = u32::from_le_bytes(size_buf);

        if size > MAX_RECORD_SIZE {
            bail!("Размер блока слишком велик: {}", size);
        }

        if &sig == b"GRUP" {
            // GRUP: size включает 24-байтовый заголовок.
            let mut grup_rest = [0u8; 16];
            reader.read_exact(&mut grup_rest)?;

            let body_size = size.saturating_sub(24) as u64;
            let group_end = reader.stream_position()? + body_size;

            process_group_body(reader, group_end, counts, 1, file_size)?;
            reader.seek(SeekFrom::Start(group_end))?;
        } else {
            // Обычная запись верхнего уровня (редко).
            let mut rec_rest = [0u8; 16];
            reader.read_exact(&mut rec_rest)?;
            reader.seek(SeekFrom::Current(size as i64))?;

            let type_str = String::from_utf8_lossy(&sig).to_string();
            *counts.entry(type_str).or_insert(0) += 1;
        }
    }

    Ok(())
}

/// Обрабатывает тело GRUP-группы до позиции `end`.
fn process_group_body<R: Read + Seek>(
    reader: &mut R,
    end: u64,
    counts: &mut BTreeMap<String, usize>,
    depth: usize,
    file_size: u64,
) -> Result<()> {
    if depth > MAX_GROUP_DEPTH {
        bail!("Слишком глубокая вложенность GRUP");
    }

    loop {
        let pos = reader.stream_position()?;
        if pos + 24 > end {
            break;
        }
        if file_size.saturating_sub(pos) < 24 {
            break;
        }

        let mut sig = [0u8; 4];
        if reader.read_exact(&mut sig).is_err() {
            break;
        }

        let mut size_buf = [0u8; 4];
        reader.read_exact(&mut size_buf)?;
        let size = u32::from_le_bytes(size_buf);

        if size > MAX_RECORD_SIZE {
            bail!("Размер блока слишком велик: {}", size);
        }

        if &sig == b"GRUP" {
            let mut grup_rest = [0u8; 16];
            reader.read_exact(&mut grup_rest)?;

            let body_size = size.saturating_sub(24) as u64;
            let subgroup_end = reader.stream_position()? + body_size;

            process_group_body(reader, subgroup_end, counts, depth + 1, file_size)?;
            reader.seek(SeekFrom::Start(subgroup_end))?;
        } else {
            let mut rec_rest = [0u8; 16];
            reader.read_exact(&mut rec_rest)?;

            let record_data_end = reader.stream_position()? + size as u64;
            if record_data_end > end {
                break;
            }

            reader.seek(SeekFrom::Current(size as i64))?;

            let type_str = String::from_utf8_lossy(&sig).to_string();
            *counts.entry(type_str).or_insert(0) += 1;
        }
    }

    Ok(())
}