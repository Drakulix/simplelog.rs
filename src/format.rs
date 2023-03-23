use std::vec::Vec;

use log::Level;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum FormatPart {
    Time,
    Level,
    Thread,
    // ThreadId,
    // ThreadName,
    Target,
    // File,
    // Line,
    Location,
    ModulePath,
    Args,
    Literal(&'static str),
}

/// output format.
#[derive(Clone, Debug)]
pub struct Format {
    pub(crate) format_parts: Vec<(Level, bool, FormatPart)>,
}

impl Format {
    fn new() -> Self {
        Self {
            format_parts: Vec::new(),
        }
    }
}

impl Default for Format {
    fn default() -> Self {
        FormatBuilder::new()
            .time_with_level_and_wrap_space(Level::Trace)
            .literal("[")
            .level()
            .literal("]")
            .thread_with_level_and_wrap_space(Level::Trace)
            .target()
            .literal(": ")
            .literal("[")
            .location()
            .literal("]")
            .args_with_level_and_wrap_space(Level::Trace)
            .build()
    }
}

/// output format builder.
pub struct FormatBuilder {
    pub(crate) format: Format,
}

use FormatPart as FP;
use Level as LV;

#[allow(dead_code)]
impl FormatBuilder {
    /// new builder.
    pub fn new() -> Self {
        Self {
            format: Format::new(),
        }
    }

    fn push(&mut self, level: Level, wrap_space: bool, part: FormatPart) -> &mut Self {
        self.format.format_parts.push((level, wrap_space, part));
        self
    }

    /// add time part.
    pub fn time(&mut self) -> &mut Self {
        self.push(LV::Trace, false, FP::Time)
    }
    /// add time part.
    pub fn time_with_level(&mut self, level: Level) -> &mut Self {
        self.push(level, false, FP::Time)
    }
    /// add time part.
    pub fn time_with_level_and_wrap_space(&mut self, level: Level) -> &mut Self {
        self.push(level, true, FP::Time)
    }

    /// add level part.
    pub fn level(&mut self) -> &mut Self {
        self.push(LV::Trace, false, FP::Level)
    }
    /// add level part.
    pub fn level_with_level(&mut self, level: Level) -> &mut Self {
        self.push(level, false, FP::Level)
    }
    /// add level part.
    pub fn level_with_level_and_wrap_space(&mut self, level: Level) -> &mut Self {
        self.push(level, true, FP::Level)
    }

    /// add thread part.
    pub fn thread(&mut self) -> &mut Self {
        self.push(LV::Trace, false, FP::Thread)
    }
    /// add thread part.
    pub fn thread_with_level(&mut self, level: Level) -> &mut Self {
        self.push(level, false, FP::Thread)
    }
    /// add thread part.
    pub fn thread_with_level_and_wrap_space(&mut self, level: Level) -> &mut Self {
        self.push(level, true, FP::Thread)
    }

    /// add target part.
    pub fn target(&mut self) -> &mut Self {
        self.push(LV::Trace, false, FP::Target)
    }
    /// add target part.
    pub fn target_with_level(&mut self, level: Level) -> &mut Self {
        self.push(level, false, FP::Target)
    }
    /// add target part.
    pub fn target_with_level_and_wrap_space(&mut self, level: Level) -> &mut Self {
        self.push(level, true, FP::Target)
    }

    /// add location  part.
    pub fn location(&mut self) -> &mut Self {
        self.push(LV::Trace, false, FP::Location)
    }
    /// add location  part.
    pub fn location_with_level(&mut self, level: Level) -> &mut Self {
        self.push(level, false, FP::Location)
    }
    /// add location  part.
    pub fn location_with_level_and_wrap_space(&mut self, level: Level) -> &mut Self {
        self.push(level, true, FP::Location)
    }

    /// add module path part.
    pub fn module_path(&mut self) -> &mut Self {
        self.push(LV::Trace, false, FP::ModulePath)
    }
    /// add module path part.
    pub fn module_path_with_level(&mut self, level: Level) -> &mut Self {
        self.push(level, false, FP::ModulePath)
    }
    /// add module path part.
    pub fn module_path_with_level_and_wrap_space(&mut self, level: Level) -> &mut Self {
        self.push(level, true, FP::ModulePath)
    }

    /// add args part.
    pub fn args(&mut self) -> &mut Self {
        self.push(LV::Trace, false, FP::Args)
    }
    /// add args part.
    pub fn args_with_level(&mut self, level: Level) -> &mut Self {
        self.push(level, false, FP::Args)
    }
    /// add args part.
    pub fn args_with_level_and_wrap_space(&mut self, level: Level) -> &mut Self {
        self.push(level, true, FP::Args)
    }

    /// add literal part.
    pub fn literal(&mut self, literal: &'static str) -> &mut Self {
        self.push(LV::Trace, false, FP::Literal(literal))
    }
    /// add literal part.
    pub fn literal_with_level(&mut self, level: Level, literal: &'static str) -> &mut Self {
        self.push(level, false, FP::Literal(literal))
    }
    /// add literal part.
    pub fn literal_with_level_and_wrap_space(
        &mut self,
        level: Level,
        literal: &'static str,
    ) -> &mut Self {
        self.push(level, true, FP::Literal(literal))
    }

    /// build.
    pub fn build(&self) -> Format {
        self.format.clone()
    }
}
