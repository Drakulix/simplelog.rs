use std::vec::Vec;

use log::Level;
#[cfg(feature = "termcolor")]
use termcolor::Color;

use crate::LevelFilter;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum FormatPartType {
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

#[derive(Clone, Debug)]
pub(crate) struct FormatPart {
    pub(crate) part_type: FormatPartType,
    pub(crate) wrap_space: bool,
    pub(crate) level_filter: LevelFilter,
    #[cfg(feature = "termcolor")]
    pub(crate) level_color: [Option<Color>; 6],
}

impl FormatPart {
    pub fn new(type_: FormatPartType) -> Self {
        Self {
            part_type: type_,
            wrap_space: false,
            // TODO: Confirm the default level.
            level_filter: LevelFilter::Trace,
            #[cfg(feature = "termcolor")]
            level_color: [None; 6],
        }
    }
}

/// output format.
#[derive(Clone, Debug)]
pub struct Format {
    pub(crate) format_parts: Vec<FormatPart>,
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
        // try to be consistent with the original format.
        FormatBuilder::new()
            .begin_time()
            .filter_level(LevelFilter::Error)
            .wrap_space(true)
            .end()
            .literal("[")
            .level()
            .literal("]")
            .begin_thread()
            .filter_level(LevelFilter::Debug)
            .wrap_space(true)
            .end()
            .begin_target()
            .filter_level(LevelFilter::Debug)
            .end()
            .begin_literal(": ")
            .filter_level(LevelFilter::Debug)
            .end()
            .literal("[")
            .begin_location()
            .filter_level(LevelFilter::Trace)
            .end()
            .literal("]")
            .begin_args()
            .wrap_space(true)
            .end()
            .build()
    }
}

/// output format builder.
pub struct FormatBuilder {
    pub(crate) format: Format,
    current_part: Option<FormatPart>,
}

#[allow(dead_code)]
impl FormatBuilder {
    /// new builder.
    pub fn new() -> Self {
        Self {
            format: Format::new(),
            current_part: None,
        }
    }

    fn push_part(&mut self, part: FormatPart) {
        self.format.format_parts.push(part)
    }

    /// Set whether the part wraps spaces.
    ///
    /// Can only be used between begin_xxx() and end().
    ///
    /// # Usage
    ///
    /// ```
    /// use simplelog::FormatBuilder;
    ///
    /// let format = FormatBuilder::new()
    ///     .begin_time()
    ///     .wrap_space(true)
    ///     .end()
    ///     .args()
    ///     .build();
    /// ```
    pub fn wrap_space(&mut self, wrap_space: bool) -> &mut Self {
        if let Some(ref mut part) = self.current_part.as_mut() {
            part.wrap_space = wrap_space;
        } else {
            // TODO: panic ?
        }
        self
    }

    /// Set the filter level for part.
    ///
    /// Can only be used between begin_xxx() and end().
    ///
    /// # Usage
    ///
    /// ```
    /// use simplelog::FormatBuilder;
    /// use simplelog::LevelFilter;
    ///
    /// let format = FormatBuilder::new()
    ///     .begin_time()
    ///     .filter_level(LevelFilter::Info)
    ///     .end()
    ///     .args()
    ///     .build();
    /// ```
    pub fn filter_level(&mut self, level_filter: LevelFilter) -> &mut Self {
        if let Some(ref mut part) = self.current_part.as_mut() {
            part.level_filter = level_filter;
        } else {
            // TODO: panic ?
        }
        self
    }

    /// Set the color of the part at level
    ///
    /// Can only be used between begin_xxx() and end().
    ///
    /// # Usage
    ///
    /// ```
    /// use simplelog::FormatBuilder;
    /// use log::Level;
    /// use termcolor::Color;
    ///
    /// let format = FormatBuilder::new()
    ///     .begin_time()
    ///     .level_color(Level::Error, Color::Red)
    ///     .end()
    ///     .args()
    ///     .build();
    /// ```
    #[cfg(feature = "termcolor")]
    pub fn level_color(&mut self, level: Level, color: Color) -> &mut Self {
        if let Some(part) = self.current_part.as_mut() {
            debug_assert!((level as usize) < part.level_color.len());
            part.level_color[level as usize] = Some(color);
        } else {
            // TODO: panic ?
        }
        self
    }

    /// begin time part.
    pub fn begin_time(&mut self) -> &mut Self {
        self.current_part = Some(FormatPart::new(FormatPartType::Time));
        self
    }

    /// begin level part.
    pub fn begin_level(&mut self) -> &mut Self {
        self.current_part = Some(FormatPart::new(FormatPartType::Level));
        self
    }

    /// begin thread part.
    pub fn begin_thread(&mut self) -> &mut Self {
        self.current_part = Some(FormatPart::new(FormatPartType::Thread));
        self
    }

    /// begin target part.
    pub fn begin_target(&mut self) -> &mut Self {
        self.current_part = Some(FormatPart::new(FormatPartType::Target));
        self
    }

    /// begin location part.
    pub fn begin_location(&mut self) -> &mut Self {
        self.current_part = Some(FormatPart::new(FormatPartType::Location));
        self
    }

    /// begin module path part.
    pub fn begin_module_path(&mut self) -> &mut Self {
        self.current_part = Some(FormatPart::new(FormatPartType::ModulePath));
        self
    }

    /// begin args part.
    pub fn begin_args(&mut self) -> &mut Self {
        self.current_part = Some(FormatPart::new(FormatPartType::Args));
        self
    }

    /// begin literal part.
    pub fn begin_literal(&mut self, literal: &'static str) -> &mut Self {
        self.current_part = Some(FormatPart::new(FormatPartType::Literal(literal)));
        self
    }

    /// end part.
    pub fn end(&mut self) -> &mut Self {
        if let Some(part) = self.current_part.take() {
            self.push_part(part)
        } else {
            // TODO: panic ?
        }
        self
    }

    /// add time part.
    ///
    /// Equivalent to:
    /// ```
    /// # use simplelog::FormatBuilder;
    /// # let mut builder = FormatBuilder::new();
    /// builder.begin_time().end();
    /// ```
    pub fn time(&mut self) -> &mut Self {
        self.begin_time().end()
    }

    /// add level part.
    ///
    /// Equivalent to:
    /// ```
    /// # use simplelog::FormatBuilder;
    /// # let mut builder = FormatBuilder::new();
    /// #[cfg(not(feature = "termcolor"))]
    /// {
    ///     builder.begin_level().end();
    /// }
    /// #[cfg(feature = "termcolor")]
    /// {
    ///     use log::Level;
    ///     use termcolor::Color;
    ///
    ///     builder.begin_level()
    ///         .level_color(Level::Error, Color::Red)
    ///         .level_color(Level::Warn, Color::Yellow)
    ///         .level_color(Level::Info, Color::Blue)
    ///         .level_color(Level::Debug, Color::Cyan)
    ///         .level_color(Level::Trace, Color::White)
    ///         .end();
    /// }
    /// ```
    pub fn level(&mut self) -> &mut Self {
        #[cfg(feature = "termcolor")]
        {
            self.begin_level()
                .level_color(Level::Error, Color::Red)
                .level_color(Level::Warn, Color::Yellow)
                .level_color(Level::Info, Color::Blue)
                .level_color(Level::Debug, Color::Cyan)
                .level_color(Level::Trace, Color::White)
                .end()
        }
        #[cfg(not(feature = "termcolor"))]
        {
            self.begin_level().end()
        }
    }

    /// add thread part.
    ///
    /// Equivalent to:
    /// ```
    /// # use simplelog::FormatBuilder;
    /// # let mut builder = FormatBuilder::new();
    /// builder.begin_thread().end();
    /// ```
    pub fn thread(&mut self) -> &mut Self {
        self.begin_thread().end()
    }

    /// add target part.
    ///
    /// Equivalent to:
    /// ```
    /// # use simplelog::FormatBuilder;
    /// # let mut builder = FormatBuilder::new();
    /// builder.begin_target().end();
    /// ```
    pub fn target(&mut self) -> &mut Self {
        self.begin_target().end()
    }

    /// add location  part.
    ///
    /// Equivalent to:
    /// ```
    /// # use simplelog::FormatBuilder;
    /// # let mut builder = FormatBuilder::new();
    /// builder.begin_location().end();
    /// ```
    pub fn location(&mut self) -> &mut Self {
        self.begin_location().end()
    }

    /// add module path part.
    ///
    /// Equivalent to:
    /// ```
    /// # use simplelog::FormatBuilder;
    /// # let mut builder = FormatBuilder::new();
    /// builder.begin_module_path().end();
    /// ```
    pub fn module_path(&mut self) -> &mut Self {
        self.begin_module_path().end()
    }

    /// add args part.
    ///
    /// Equivalent to:
    /// ```
    /// # use simplelog::FormatBuilder;
    /// # let mut builder = FormatBuilder::new();
    /// builder.begin_args().end();
    /// ```
    pub fn args(&mut self) -> &mut Self {
        self.begin_args().end()
    }

    /// add literal part.
    ///
    /// Equivalent to:
    /// ```
    /// # use simplelog::FormatBuilder;
    /// # let mut builder = FormatBuilder::new();
    /// # let literal = "literal";
    /// builder.begin_literal(literal).end();
    /// ```
    pub fn literal(&mut self, literal: &'static str) -> &mut Self {
        self.begin_literal(literal).end()
    }

    /// build.
    pub fn build(&self) -> Format {
        self.format.clone()
    }
}
