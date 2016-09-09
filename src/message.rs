use std::fmt;

pub enum Message {
    Raw(String),
    Info(String),
}

impl fmt::Display for Message {
    /// If a Message is of type Raw, it's inner str is not going
    /// to be altered when using the `format!` macro.
    ///
    /// # Example
    ///
    /// ```
    /// use slate::message::Message;
    ///
    /// let message = Message::Raw("don't touch me".to_string());
    /// print!("{}", message);
    /// //=> "don't touch me"
    /// ```
    ///
    /// In the case the message is of type Info, it will have
    /// an unique eol.
    ///
    /// # Example
    ///
    /// ```
    /// use slate::message::Message;
    ///
    /// let message = Message::Info("hello world".to_string());
    /// print!("{}", message);
    /// //=> "hello world\n"
    /// ```
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Message::Raw(ref msg) => write!(formatter, "{}", msg),
            &Message::Info(ref msg) => write!(formatter, "{}\n", msg.trim_right()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_does_not_touch_raw_messages() {
        let expected = "don't touch me".to_string();
        let subject = Message::Raw("don't touch me".to_string());
        let actual = format!("{}", subject);

        assert_eq!(expected, actual);
    }

    #[test]
    fn it_adds_eol_to_info_messages() {
        let expected = "you can touch me\n".to_string();
        let subject = Message::Info("you can touch me".to_string());
        let actual = format!("{}", subject);

        assert_eq!(expected, actual);
    }

    #[test]
    fn it_does_not_add_multiple_eol_to_info_messages() {
        let expected = "you can touch me\n".to_string();
        let subject = Message::Info("you can touch me\n".to_string());
        let actual = format!("{}", subject);

        assert_eq!(expected, actual);
    }
}
