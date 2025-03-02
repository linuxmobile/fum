use crossterm::event::KeyCode;
use serde::{de, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Keybind {
    Backspace,
    Enter,
    Left,
    Up,
    Right,
    Down,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    Esc,
    Caps,
    F(u8),
    Char(char),
    Many(Vec<Keybind>)
}

impl<'de> Deserialize<'de> for Keybind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let keybind: &str = Deserialize::deserialize(deserializer)?;

        if keybind.contains(";") {
            let keybinds = keybind
                .split(";")
                .filter(|k| !k.is_empty())
                .map(|k| Keybind::parse_keybind(k.trim()))
                .collect::<Result<Vec<Keybind>, D::Error>>()?;

            return Ok(Keybind::Many(keybinds));
        }

        Keybind::parse_keybind(keybind)
    }
}

impl Keybind {
    pub fn into_keycode(&self) -> KeyCode {
        match self {
            Keybind::Backspace           => KeyCode::Backspace,
            Keybind::Enter               => KeyCode::Enter,
            Keybind::Left                => KeyCode::Left,
            Keybind::Up                  => KeyCode::Up,
            Keybind::Right               => KeyCode::Right,
            Keybind::Down                => KeyCode::Down,
            Keybind::End                 => KeyCode::End,
            Keybind::PageUp              => KeyCode::PageUp,
            Keybind::PageDown            => KeyCode::PageDown,
            Keybind::Tab                 => KeyCode::Tab,
            Keybind::BackTab             => KeyCode::BackTab,
            Keybind::Delete              => KeyCode::Delete,
            Keybind::Insert              => KeyCode::Insert,
            Keybind::Esc                 => KeyCode::Esc,
            Keybind::Caps                => KeyCode::CapsLock,
            Keybind::F(u8)               => KeyCode::F(*u8),
            Keybind::Char(char)          => KeyCode::Char(*char),
            Keybind::Many(_)             => unreachable!()
        }
    }

    fn parse_keybind<D>(keybind: &str) -> Result<Keybind, D>
    where
        D: de::Error
    {
        match keybind {
            "backspace"     => Ok(Keybind::Backspace),
            "enter"         => Ok(Keybind::Enter),
            "left"          => Ok(Keybind::Left),
            "up"            => Ok(Keybind::Up),
            "right"         => Ok(Keybind::Right),
            "down"          => Ok(Keybind::Down),
            "end"           => Ok(Keybind::End),
            "page_up"       => Ok(Keybind::PageUp),
            "page_down"     => Ok(Keybind::PageDown),
            "tab"           => Ok(Keybind::Tab),
            "back_tab"      => Ok(Keybind::BackTab),
            "delete"        => Ok(Keybind::Delete),
            "insert"        => Ok(Keybind::Insert),
            "caps"          => Ok(Keybind::Caps),
            "esc"           => Ok(Keybind::Esc),
            k if k.starts_with('f') => {
                match k[1..].parse::<u8>() {
                    Ok(fn_num) => Ok(Keybind::F(fn_num)),
                    Err(_) => Err(de::Error::custom("Invalid fn key format"))
                }
            },
            k if k.len() == 1 => {
                match k.chars().next() {
                    Some(char) => Ok(Keybind::Char(char)),
                    None => Err(de::Error::custom(format!("Invalid keyboard key: {k}")))
                }
            },
            _ => Err(de::Error::custom(format!("Unknown keybind: {keybind}")))
        }
    }
}
