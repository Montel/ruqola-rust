/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
use serde::Deserialize;
enum AttachmentType {
    Unknown,
    NormalText,
    File,
    Video,
    Audio,
    Image,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct MessageAttachment {
    pub title: String,
    pub description: String,
    pub color: String,
    pub link: String,
    pub author_name: String,
    pub author_icon: String,
    pub image_type: String,
}

/*
class Messageattachment {
  List<MessageAttachmentField>? mAttachmentFields;
  AttachmentType mAttachmentType = AttachmentType.unknown;
  String? mColor;
  String? mDescription;
  String? mTitle;
  String? mLink;
  String? mAuthorName;
  String? mMimeType;
  String? mText;
  String? mAttachmentId;
  String? mAuthorIcon;
  String? mAttachmentFieldsText;
  String? mImageUrlPreview;
  String? mAttachmentGeneratedTitle;
  int mAttachmentSize = -1;
  int mImageHeight = -1;
  int mImageWidth = -1;
  bool mIsAnimatedImage = false;
  bool mCollapsed = false;
  bool mShowAttachment = false;
}
*/
