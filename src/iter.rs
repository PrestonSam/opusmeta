//! Module containing iterator types.

use crate::Picture;
use crate::Result;

type CommentHashIter<'a> = std::collections::hash_map::Iter<'a, String, Vec<String>>;

type CommentsExceptPicturesIter<'a> =
    std::iter::Filter<CommentHashIter<'a>, fn(&(&String, &Vec<String>)) -> bool>;

/// An iterator over the comments of an opus file, excluding pictures.
///
/// The iterator's Item is `(&'a str, Vec<&'a str>)`.
/// This iterator immutably borrows the tags stored in the [`Tag`](crate::Tag) struct.
pub struct CommentsIterator<'a> {
    pub(crate) comments_iter: CommentsExceptPicturesIter<'a>,
}

impl<'a> Iterator for CommentsIterator<'a> {
    type Item = (&'a str, Vec<&'a str>);

    fn next(&mut self) -> Option<Self::Item> {
        self.comments_iter.next().map(|(key, vals)| {
            let key = key.as_ref();
            let values = vals.iter().map(AsRef::as_ref).collect();

            (key, values)
        })
    }
}

/// An iterator over the pictures stored in the comments.
///
/// The iterator Item is `Result<Picture>`, containing an `Error` should the given image fail to decode.
/// This iterator immutably borrows the tags stored in the [`Tag`](crate::Tag) struct.
pub struct PicturesIterator<'a> {
    pub(crate) pictures_iter: core::slice::Iter<'a, String>,
}

impl Iterator for PicturesIterator<'_> {
    type Item = Result<Picture>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pictures_iter.next().map(|s| Picture::from_base64(s))
    }
}
