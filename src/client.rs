/// Describes a generic network client used to fetch Wordle data from upstream
/// network sources.
///
/// The [`Client`] has no provided implementation so that this crate
/// can remain dependency-free. It is expected that consumers of this crate will
/// provide their own clients if necessary.
///
/// # Examples
///
/// For examples, see [`Wordle`].
///
/// [`Wordle`]: [crate::Wordle]
pub trait Client {
    /// Date representation for looking up answers by date.
    /// Arbitrary. Use of this type is left up to the implementer.
    type Date;

    /// The error type returned by the client's methods.
    type Error: std::error::Error + 'static;

    /// Fetch the Wordle answer for the given date.
    ///
    /// # Errors
    ///
    /// Should the network request return an error, that should be returned. When the [`Client`] is
    /// used by [`Wordle`], the error returned by this method will be wrapped by
    /// [`wordle::Error::Safe`].
    ///
    /// [`Wordle`]: crate::Wordle
    /// [`wordle::Error::Safe`]: crate::Error::Safe
    fn fetch_answer(&self, date: Self::Date) -> Result<String, Self::Error>;

    /// Fetch the list of all possible answers for wordle.
    ///
    /// # Errors
    ///
    /// Should the network request return an error, that should be returned. When the [`Client`] is
    /// used by [`Wordle`], the error returned by this method will be wrapped by
    /// [`wordle::Error::Safe`].
    ///
    /// [`Wordle`]: crate::Wordle
    /// [`wordle::Error::Safe`]: crate::Error::Safe
    fn fetch_words(&self) -> Result<impl Iterator<Item = String> + 'static, Self::Error>;
}
