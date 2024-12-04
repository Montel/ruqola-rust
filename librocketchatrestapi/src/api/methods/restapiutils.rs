/*
 * SPDX-FileCopyrightText: 2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

fn adapt_url(url: String) -> String {
    if url.is_empty() {
        return url;
    }
    // Avoid to add more https:// :)
    if url.starts_with("https://") || url.starts_with("http://") {
        url
    } else {
        let new_url = String::from("https://") + &url;
        new_url
    }
}

fn generateUrl(
    serverUrl: String,
    restApiUrlType: RestApiUrlType,
    restApiUrlExtensionType: RestApiUrlExtensionType,
) -> String {
    if serverUrl.is_empty() {
        return String::from("");
    }
    /*
        String urlStr =
        adaptUrl(serverUrl) + restApiUrlExtensionType.path + type.name;
    if (urlExtension.isNotEmpty) {
      urlStr += '/$urlExtension';
    }
    */

    String::from("")
}

#[derive(Debug)]
enum RestApiUrlExtensionType {
    NoExtension = 0,
    V1 = 1,
    Apps = 2,
}

impl RestApiUrlExtensionType {
    fn path(&self) -> String {
        match self {
            RestApiUrlExtensionType::NoExtension => "/api/".to_string(),
            RestApiUrlExtensionType::V1 => "/api/v1/".to_string(),
            RestApiUrlExtensionType::Apps => "/api/apps/".to_string(),
        }
    }
}

#[derive(Debug)]
enum RestApiUrlType {
    Login = 0,
    Logout = 1,
    Me = 2,
    UsersCreate = 3,
    UsersDelete = 4,
    UsersCreateToken = 5,
    UsersGetPresence = 6,
    UsersInfo = 7,
    UsersList = 8,
    UsersRegister = 9,
    UsersGetAvatar = 10,
    UsersResetAvatar = 11,
    UsersSetAvatar = 12,
    UsersUpdate = 13,
    UsersPresence = 14, /*since 1.1.0*/
    UsersUpdateOwnBasicInfo = 15,

    // PersonalAccess Token
    UsersGeneratePersonalAccessToken = 16,
    UsersRegeneratePersonalAccessToken = 17,
    UsersRemovePersonalAccessToken = 18,
    UsersGetPersonalAccessTokens = 19,

    UsersGetUsernameSuggestion = 20,
    UsersSetStatus = 21,
    UsersGetStatus = 22,
    /* ... 2.4 ? */
    UsersAutocomplete = 23,
    /* ... 3.1 ? */
    UsersRemoveOtherTokens = 24,
    UsersDeleteOwnAccount = 25,
    UsersSetActiveStatus = 26,

    UsersResetTOTP = 27,
    UsersResetE2EKey = 28,

    ChatDelete = 29,
    ChatGetMessage = 30,
    ChatPinMessage = 31,
    ChatPostMessage = 32,
    ChatReact = 33,
    ChatSearch = 34,
    ChatStarMessage = 35,
    ChatUnPinMessage = 36,
    ChatUnStarMessage = 37,
    ChatUpdate = 38,
    ChatIgnoreUser = 39,
    ChatReportMessage = 40,
    // Since 1.0.0
    ChatFollowMessage = 41,
    ChatUnFollowMessage = 42,
    ChatGetThreadsList = 43,
    ChatSyncThreadMessages = 44,
    ChatGetThreadMessages = 45,
    ChatSyncThreadsList = 46,
    ChatGetDeletedMessage = 47,
    ChatSendMessage = 48,
    ChatMessageReadReceipts = 49,
    // Since 2.0.0
    ChatGetPinnedMessages = 50,
    ChatGetMentionedMessages = 51,

    // Since 2.2.x
    ChatGetStarredMessages = 52,
    ChatGetSnippetedMessages = 53,

    ChatSyncMessages = 54,

    ChannelsAddAll = 55,
    //@since 0.75
    ChannelsAddLeader = 56,
    ChannelsAddModerator = 57,
    ChannelsAddOwner = 58,
    ChannelsArchive = 59,
    ChannelsClose = 60,
    ChannelsCreate = 61,
    ChannelsGetIntegrations = 62,
    ChannelsHistory = 63,
    ChannelsInfo = 64,
    ChannelsInvite = 65,
    ChannelsKick = 66,
    ChannelsLeave = 67,
    ChannelsList = 68,
    ChannelsListJoined = 69,
    ChannelsOpen = 70,
    ChannelsRemoveModerator = 71,
    ChannelsRemoveOwner = 72,
    //@since 0.75
    ChannelsRemoveLeader = 73,
    ChannelsRename = 74,
    ChannelsSetDescription = 75,
    ChannelsSetJoinCode = 76,
    ChannelsSetPurpose = 77,
    ChannelsSetReadOnly = 78,
    ChannelsSetTopic = 79,
    ChannelsSetType = 80,
    ChannelsSetAnnouncement = 81,
    ChannelsFiles = 82,
    ChannelsUnarchive = 83,
    ChannelsCounters = 84,
    ChannelsMembers = 85,
    ChannelsJoin = 86,
    ChannelsOnline = 87,
    // since 0.70
    ChannelsModerators = 88,
    // Since 0.71
    ChannelsDelete = 89,
    // since 0.65
    ChannelsRoles = 90,
    // since 0.63
    ChannelsGetAllUserMentionsByChannel = 91,
    GroupsAddAll = 92,
    GroupsAddModerator = 93,
    GroupsAddOwner = 94,
    GroupsAddLeader = 95,
    GroupsArchive = 96,
    GroupsClose = 97,
    GroupsCreate = 98,
    // since 0.70
    GroupsDelete = 99,
    GroupsGetIntegrations = 100,
    GroupsHistory = 101,
    GroupsInfo = 102,
    GroupsInvite = 103,
    GroupsKick = 104,
    GroupsLeave = 105,
    GroupsList = 106,
    GroupsOpen = 107,
    GroupsRemoveModerator = 108,
    GroupsRemoveOwner = 109,
    GroupsRemoveLeader = 110,
    GroupsRename = 111,
    GroupsSetDescription = 112,
    GroupsSetPurpose = 113,
    GroupsSetReadOnly = 114,
    GroupsSetTopic = 115,
    GroupsSetType = 116,
    GroupsUnarchive = 117,
    GroupsSetAnnouncement = 118,
    GroupsFiles = 119,
    GroupsListAll = 120,
    GroupsMembers = 121,
    // Since 0.65
    GroupsRoles = 122,
    GroupsCounters = 123,
    GroupsSetEncrypted = 124,
    ServerInfo = 125,
    Settings = 126,
    SettingsPublic = 127,
    UpdateAdminSettings = 128,
    Spotlight = 129,
    LoadEmojiCustom = 130,
    EmojiCustomDelete = 131,
    EmojiCustomCreate = 132,
    EmojiCustomUpdate = 133,
    EmojiCustomAll = 134,

    RoomsUpload = 135,
    RoomsSaveNotification = 136,
    RoomsSaveSettings = 137,
    RoomsAdminRooms = 138,
    RoomsAdminRoomsGetRoom = 139,
    ImClose = 140,
    ImCreate = 141,
    ImHistory = 142,
    ImFiles = 143,
    ImOpen = 144,
    ImMessages = 145,
    ImDelete = 146,
    // Since 0.59
    ImMembers = 147,
    ImSetTopic = 148,
    SubscriptionsRead = 149,
    SubscriptionsUnRead = 150,
    SettingsOauth = 151,
    SettingsAddCustomOauth = 152,
    RoomsGet = 153,
    RoomsFavorite = 154,
    RoomsCleanHistory = 155,
    RoomsInfo = 156,
    RoomsLeave = 157,
    RoomsCreateDiscussion = 158,
    RoomsGetDiscussions = 159,
    RoomsExport = 160,
    RoomsChangeArchivactionState = 161,
    RoomsNameExists = 162,

    ForgotPassword = 163,
    CommandsList = 164,
    CommandsGet = 165,
    CommandsRun = 166,
    CommandsPreview = 167,
    // since 0.70
    E2ESetRoomKeyID = 168,
    E2EfetchMyKeys = 169,
    E2EupdateGroupKey = 170,
    E2ESetUserPublicAndPrivateKeys = 171,
    E2EGetUsersOfRoomWithoutKey = 172,
    E2ERequestSubscriptionKeys = 173,
    E2EResetOwnE2EKey = 174,

    RolesList = 175,
    RolesCreate = 176,
    RolesUpdate = 177,
    RolesSync = 178,
    RolesDelete = 179,
    RolesAddUserToRole = 180,
    RolesGetUsersInRole = 181,
    RolesRemoveUserFromRole = 182,

    // since 0.74
    VideoConfJitsi = 183,

    // Autotranslate @since 2.0
    AutoTranslateGetSupportedLanguages = 184,
    AutoTranslateSaveSettings = 185,
    AutoTranslateTranslateMessage = 186,

    // Since 2.4
    CustomUserStatusList = 187,
    CustomUserStatusCreate = 188,
    CustomUserStatusDelete = 189,
    CustomUserStatusUpdate = 190,
    CustomSoundsList = 191,

    // Invite since 2.5 ?
    FindOrCreateInvite = 192,
    ListInvites = 193,
    RemoveInvite = 194,
    UseInviteToken = 195,
    ValidateInviteToken = 196,

    // RC 6.0
    SendInvitationEmails = 197,

    // Since 2.4.0
    RoomsAutocompleteChannelAndPrivate = 198,

    // two factor authentication
    Users2FASendEmailCode = 199,
    Users2FAEnableEmail = 200,
    Users2FADisableEmail = 201,

    UsersSetPreferences = 202,

    // Permission
    PermissionsListAll = 203,
    PermissionsUpdate = 204,

    Statistics = 205,

    Directory = 206,

    // Teams 3.13
    TeamsList = 207,
    TeamsListAll = 208,
    TeamsCreate = 209,
    TeamsAddRooms = 210,
    TeamsRemoveRoom = 211,
    TeamsUpdateRoom = 212,
    TeamsListRooms = 213,
    TeamsListRoomsOfUser = 214,
    TeamsMembers = 215,
    TeamsAddMembers = 216,
    TeamsUpdateMember = 217,
    TeamsRemoveMember = 218,
    TeamsLeave = 219,
    TeamsInfo = 220,
    TeamsDelete = 221,
    TeamsAutocomplete = 222,
    TeamsConvertToChannel = 223,
    RoomsAutocompleteAvailableForTeams = 224,
    ChannelsConvertToTeam = 225,
    GroupsConvertToTeam = 226,

    StdoutQueue = 227,

    // Oauth
    OauthAppsList = 228,
    OauthAppsGet = 229,
    // 5.4.0
    OauthAppsCreate = 230,
    OauthAppsUpdate = 231,
    OauthAppsDelete = 232,

    // License
    LicensesGet = 233,
    LicensesMaxActiveUsers = 234,
    LicensesAdd = 235,
    LicensesIsEntreprise = 236,
    // 6.5.0
    LicensesInfo = 237,

    // Banner
    BannersDismiss = 238,
    Banners = 239,

    // Session
    SessionsList = 240,
    SessionsInfo = 241,
    SessionsListAll = 242,
    SessionsLogoutMe = 243,
    SessionsLogout = 244,
    SessionsInfoAdmin = 245,

    UserRequestDataDownload = 246,

    // 5.4.0
    RoomsDelete = 247,

    // Video Conference
    VideoConferenceStart = 248,
    VideoConferenceJoin = 249,
    VideoConferenceCancel = 250,
    VideoConferenceInfo = 251,
    VideoConferenceList = 252,
    VideoConferenceProviders = 253,
    VideoConferenceCapabilities = 254,

    // Moderation
    ModerationReportsByUsers = 255,
    ModerationDismissReports = 256,
    ModerationUserReportedMessages = 257,
    ModerationUserDeleteReportedMessages = 258,
    ModerationReports = 259,
    ModerationReportInfo = 260,
    ModerationUserReports = 261,
    ModerationUserReportsByUserId = 262,
    ModerationDismissUserReports = 263,
    ModerationReportUser = 264,

    // Proxy
    MethodCall = 265,
    MethodCallAnon = 266,

    // Apps.ui.interaction
    AppsUiInteraction = 267,

    // users.logoutOtherClients
    UsersLogoutOtherClients = 268,

    // rooms.muteUser
    RoomsMuteUser = 269,
    RoomsUnmuteUser = 270,

    // /api/apps/ (applications)
    FeaturedApps = 271,
    CategoriesApps = 272,
    CountApps = 273,
    NotifyAdminsApps = 274,
    InstalledApps = 275,
    MarketplaceApps = 276,

    // users.listByStatus
    UsersListByStatus = 277,

    // users.sendWelcomeEmail
    UsersSendWelcomeEmail = 278,
}

impl RestApiUrlType {
    fn path(&self) -> String {
        match self {
            RestApiUrlType::Login => "login".to_string(),
            RestApiUrlType::Logout => "logout".to_string(),
            RestApiUrlType::Me => "me".to_string(),
            RestApiUrlType::UsersGetAvatar => "users.getAvatar".to_string(),
            RestApiUrlType::UsersDelete => "users.delete".to_string(),
            RestApiUrlType::UsersCreate => "users.create".to_string(),
            RestApiUrlType::UsersCreateToken => "users.createToken".to_string(),
            RestApiUrlType::UsersGetPresence => "users.getPresence".to_string(),
            RestApiUrlType::UsersInfo => "users.info".to_string(),
            RestApiUrlType::UsersList => "users.list".to_string(),
            RestApiUrlType::UsersRegister => "users.register".to_string(),
            RestApiUrlType::UsersResetAvatar => "users.resetAvatar".to_string(),
            RestApiUrlType::UsersSetAvatar => "users.setAvatar".to_string(),
            RestApiUrlType::UsersUpdate => "users.update".to_string(),
            RestApiUrlType::UsersGetUsernameSuggestion => "users.getUsernameSuggestion".to_string(),

            RestApiUrlType::UsersRemovePersonalAccessToken => {
                "users.removePersonalAccessToken".to_string()
            }
            RestApiUrlType::UsersGeneratePersonalAccessToken => {
                "users.generatePersonalAccessToken".to_string()
            }
            RestApiUrlType::UsersRegeneratePersonalAccessToken => {
                "users.regeneratePersonalAccessToken".to_string()
            }
            RestApiUrlType::UsersGetPersonalAccessTokens => {
                "users.getPersonalAccessTokens".to_string()
            }

            RestApiUrlType::UsersPresence => "users.presence".to_string(),
            RestApiUrlType::UsersUpdateOwnBasicInfo => "users.updateOwnBasicInfo".to_string(),
            RestApiUrlType::UsersSetStatus => "users.setStatus".to_string(),
            RestApiUrlType::UsersGetStatus => "users.getStatus".to_string(),
            RestApiUrlType::UsersAutocomplete => "users.autocomplete".to_string(),
            RestApiUrlType::UsersRemoveOtherTokens => "users.removeOtherTokens".to_string(),
            RestApiUrlType::UsersSetActiveStatus => "users.setActiveStatus".to_string(),
            RestApiUrlType::UsersResetTOTP => "users.resetTOTP".to_string(),
            RestApiUrlType::UsersResetE2EKey => "users.resetE2EKey".to_string(),

            RestApiUrlType::ChatDelete => "chat.delete".to_string(),
            RestApiUrlType::ChatGetMessage => "chat.getMessage".to_string(),
            RestApiUrlType::ChatPinMessage => "chat.pinMessage".to_string(),
            RestApiUrlType::ChatPostMessage => "chat.postMessage".to_string(),
            RestApiUrlType::ChatReact => "chat.react".to_string(),
            RestApiUrlType::ChatStarMessage => "chat.starMessage".to_string(),
            RestApiUrlType::ChatUnPinMessage => "chat.unPinMessage".to_string(),
            RestApiUrlType::ChatUnStarMessage => "chat.unStarMessage".to_string(),
            RestApiUrlType::ChatUpdate => "chat.update".to_string(),
            RestApiUrlType::ChatSearch => "chat.search".to_string(),
            RestApiUrlType::ChatIgnoreUser => "chat.ignoreUser".to_string(),
            RestApiUrlType::ChatReportMessage => "chat.reportMessage".to_string(),
            RestApiUrlType::ChatFollowMessage => "chat.followMessage".to_string(),
            RestApiUrlType::ChatUnFollowMessage => "chat.unfollowMessage".to_string(),
            RestApiUrlType::ChatGetDeletedMessage => "chat.getDeletedMessages".to_string(),
            RestApiUrlType::ChatMessageReadReceipts => "chat.getMessageReadReceipts".to_string(),
            RestApiUrlType::ChatSyncThreadsList => "chat.syncThreadsList".to_string(),
            RestApiUrlType::ChatGetThreadsList => "chat.getThreadsList".to_string(),
            RestApiUrlType::ChatSyncThreadMessages => "chat.syncThreadMessages".to_string(),
            RestApiUrlType::ChatGetThreadMessages => "chat.getThreadMessages".to_string(),
            RestApiUrlType::ChatSendMessage => "chat.sendMessage".to_string(),
            RestApiUrlType::ChatGetPinnedMessages => "chat.getPinnedMessages".to_string(),
            RestApiUrlType::ChatGetMentionedMessages => "chat.getMentionedMessages".to_string(),
            RestApiUrlType::ChatGetStarredMessages => "chat.getStarredMessages".to_string(),
            RestApiUrlType::ChatGetSnippetedMessages => "chat.getSnippetedMessages".to_string(),
            RestApiUrlType::ChatSyncMessages => "chat.syncMessages".to_string(),
            RestApiUrlType::ChannelsAddAll => "channels.addAll".to_string(),
            RestApiUrlType::ChannelsAddModerator => "channels.addModerator".to_string(),
            RestApiUrlType::ChannelsAddOwner => "channels.addOwner".to_string(),
            RestApiUrlType::ChannelsAddLeader => "channels.addLeader".to_string(),
            RestApiUrlType::ChannelsArchive => "channels.archive".to_string(),
            RestApiUrlType::ChannelsModerators => "channels.moderators".to_string(),
            RestApiUrlType::ChannelsClose => "channels.close".to_string(),
            RestApiUrlType::ChannelsCreate => "channels.create".to_string(),
            RestApiUrlType::ChannelsGetIntegrations => "channels.getIntegrations".to_string(),
            RestApiUrlType::ChannelsHistory => "channels.history".to_string(),
            RestApiUrlType::ChannelsInfo => "channels.info".to_string(),
            RestApiUrlType::ChannelsInvite => "channels.invite".to_string(),
            RestApiUrlType::ChannelsKick => "channels.kick".to_string(),
            RestApiUrlType::ChannelsLeave => "channels.leave".to_string(),
            RestApiUrlType::ChannelsList => "channels.list".to_string(),
            RestApiUrlType::ChannelsListJoined => "channels.list.joined".to_string(),
            RestApiUrlType::ChannelsOpen => "channels.open".to_string(),
            RestApiUrlType::ChannelsRemoveModerator => "channels.removeModerator".to_string(),
            RestApiUrlType::ChannelsRemoveOwner => "channels.removeOwner".to_string(),
            RestApiUrlType::ChannelsRemoveLeader => "channels.removeLeader".to_string(),
            RestApiUrlType::ChannelsRename => "channels.rename".to_string(),
            RestApiUrlType::ChannelsSetDescription => "channels.setDescription".to_string(),
            RestApiUrlType::ChannelsSetJoinCode => "channels.setJoinCode".to_string(),
            RestApiUrlType::ChannelsSetPurpose => "channels.setPurpose".to_string(),
            RestApiUrlType::ChannelsSetReadOnly => "channels.setReadOnly".to_string(),
            RestApiUrlType::ChannelsSetTopic => "channels.setTopic".to_string(),
            RestApiUrlType::ChannelsSetAnnouncement => "channels.setAnnouncement".to_string(),
            RestApiUrlType::ChannelsFiles => "channels.files".to_string(),
            RestApiUrlType::ChannelsSetType => "channels.setType".to_string(),
            RestApiUrlType::ChannelsUnarchive => "channels.unarchive".to_string(),
            RestApiUrlType::ChannelsRoles => "channels.roles".to_string(),
            RestApiUrlType::ChannelsCounters => "channels.counters".to_string(),
            RestApiUrlType::ChannelsJoin => "channels.join".to_string(),
            RestApiUrlType::ChannelsMembers => "channels.members".to_string(),
            RestApiUrlType::ChannelsDelete => "channels.delete".to_string(),
            RestApiUrlType::ChannelsOnline => "channels.online".to_string(),
            RestApiUrlType::ChannelsGetAllUserMentionsByChannel => {
                "channels.getAllUserMentionsByChannel".to_string()
            }
            RestApiUrlType::GroupsAddAll => "groups.addAll".to_string(),
            RestApiUrlType::GroupsAddModerator => "groups.addModerator".to_string(),
            RestApiUrlType::GroupsAddOwner => "groups.addOwner".to_string(),
            RestApiUrlType::GroupsArchive => "groups.archive".to_string(),
            RestApiUrlType::GroupsClose => "groups.close".to_string(),
            RestApiUrlType::GroupsCreate => "groups.create".to_string(),
            RestApiUrlType::GroupsGetIntegrations => "groups.getIntegrations".to_string(),
            RestApiUrlType::GroupsHistory => "groups.history".to_string(),
            RestApiUrlType::GroupsInfo => "groups.info".to_string(),
            RestApiUrlType::GroupsInvite => "groups.invite".to_string(),
            RestApiUrlType::GroupsKick => "groups.kick".to_string(),
            RestApiUrlType::GroupsLeave => "groups.leave".to_string(),
            RestApiUrlType::GroupsList => "groups.list".to_string(),
            RestApiUrlType::GroupsOpen => "groups.open".to_string(),
            RestApiUrlType::GroupsRemoveModerator => "groups.removeModerator".to_string(),
            RestApiUrlType::GroupsRemoveOwner => "groups.removeOwner".to_string(),
            RestApiUrlType::GroupsRename => "groups.rename".to_string(),
            RestApiUrlType::GroupsSetDescription => "groups.setDescription".to_string(),
            RestApiUrlType::GroupsSetPurpose => "groups.setPurpose".to_string(),
            RestApiUrlType::GroupsSetReadOnly => "groups.setReadOnly".to_string(),
            RestApiUrlType::GroupsSetTopic => "groups.setTopic".to_string(),
            RestApiUrlType::GroupsSetType => "groups.setType".to_string(),
            RestApiUrlType::GroupsUnarchive => "groups.unarchive".to_string(),
            RestApiUrlType::GroupsSetAnnouncement => "groups.setAnnouncement".to_string(),
            RestApiUrlType::GroupsFiles => "groups.files".to_string(),
            RestApiUrlType::GroupsRoles => "groups.roles".to_string(),
            RestApiUrlType::GroupsCounters => "groups.counters".to_string(),
            RestApiUrlType::GroupsRemoveLeader => "groups.removeLeader".to_string(),
            RestApiUrlType::GroupsAddLeader => "groups.addLeader".to_string(),
            RestApiUrlType::GroupsDelete => "groups.delete".to_string(),
            RestApiUrlType::GroupsListAll => "groups.listAll".to_string(),
            RestApiUrlType::GroupsMembers => "groups.members".to_string(),
            RestApiUrlType::GroupsSetEncrypted => "groups.setEncrypted".to_string(),
            RestApiUrlType::ServerInfo => "info".to_string(),
            RestApiUrlType::Settings => "settings".to_string(),
            RestApiUrlType::SettingsPublic => "settings.public".to_string(),
            RestApiUrlType::UpdateAdminSettings => "settings".to_string(),
            RestApiUrlType::RoomsUpload => "rooms.upload".to_string(),
            RestApiUrlType::RoomsSaveNotification => "rooms.saveNotification".to_string(),
            RestApiUrlType::RoomsSaveSettings => "rooms.saveRoomSettings".to_string(),
            RestApiUrlType::RoomsAdminRooms => "rooms.adminRooms".to_string(),
            RestApiUrlType::RoomsAdminRoomsGetRoom => "rooms.adminRooms.getRoom".to_string(),
            RestApiUrlType::Spotlight => "spotlight".to_string(),
            RestApiUrlType::ImClose => "im.close".to_string(),
            RestApiUrlType::ImCreate => "im.create".to_string(),
            RestApiUrlType::ImOpen => "im.open".to_string(),
            RestApiUrlType::ImSetTopic => "im.setTopic".to_string(),
            RestApiUrlType::ImHistory => "im.history".to_string(),
            RestApiUrlType::ImFiles => "im.files".to_string(),
            RestApiUrlType::ImDelete => "im.delete".to_string(),
            RestApiUrlType::ImMessages => "im.messages".to_string(),
            RestApiUrlType::ImMembers => "im.members".to_string(),
            RestApiUrlType::LoadEmojiCustom => "emoji-custom.list".to_string(),
            RestApiUrlType::EmojiCustomDelete => "emoji-custom.delete".to_string(),
            RestApiUrlType::EmojiCustomCreate => "emoji-custom.create".to_string(),
            RestApiUrlType::EmojiCustomUpdate => "emoji-custom.update".to_string(),
            RestApiUrlType::EmojiCustomAll => "emoji-custom.all".to_string(),
            RestApiUrlType::SubscriptionsRead => "subscriptions.read".to_string(),
            RestApiUrlType::SubscriptionsUnRead => "subscriptions.unread".to_string(),
            RestApiUrlType::RoomsGet => "rooms.get".to_string(),
            RestApiUrlType::RoomsFavorite => "rooms.favorite".to_string(),
            RestApiUrlType::RoomsCleanHistory => "rooms.cleanHistory".to_string(),
            // since 1.0.0
            RestApiUrlType::RoomsCreateDiscussion => "rooms.createDiscussion".to_string(),
            RestApiUrlType::RoomsGetDiscussions => "rooms.getDiscussions".to_string(),

            // since 3.8.0
            RestApiUrlType::RoomsExport => "rooms.export".to_string(),
            RestApiUrlType::RoomsChangeArchivactionState => {
                "rooms.changeArchivationState".to_string()
            }
            // since 0.72 ? Need to implement it
            RestApiUrlType::RoomsInfo => "rooms.info".to_string(),
            RestApiUrlType::RoomsLeave => "rooms.leave".to_string(),
            // Since 5.4.0
            RestApiUrlType::RoomsDelete => "rooms.delete".to_string(),

            //
            RestApiUrlType::ForgotPassword => "users.forgotPassword".to_string(),
            RestApiUrlType::CommandsList => "commands.list".to_string(),
            RestApiUrlType::CommandsGet => "commands.get".to_string(),
            RestApiUrlType::CommandsRun => "commands.run".to_string(),
            RestApiUrlType::CommandsPreview => "commands.preview".to_string(),
            RestApiUrlType::E2EfetchMyKeys => "e2e.fetchMyKeys".to_string(),
            RestApiUrlType::E2EupdateGroupKey => "e2e.updateGroupKey".to_string(),

            RestApiUrlType::E2ESetRoomKeyID => "e2e.setRoomKeyID".to_string(),
            RestApiUrlType::E2ESetUserPublicAndPrivateKeys => {
                "e2e.setUserPublicAndPrivateKeys".to_string()
            }
            RestApiUrlType::E2EGetUsersOfRoomWithoutKey => {
                "e2e.getUsersOfRoomWithoutKey".to_string()
            }
            RestApiUrlType::E2ERequestSubscriptionKeys => "e2e.requestSubscriptionKeys".to_string(),
            RestApiUrlType::E2EResetOwnE2EKey => "e2e.resetOwnE2EKey".to_string(),

            RestApiUrlType::RolesList => "roles.list".to_string(),
            RestApiUrlType::RolesCreate => "roles.create".to_string(),
            RestApiUrlType::RolesSync => "roles.sync".to_string(),
            RestApiUrlType::RolesUpdate => "roles.update".to_string(),
            RestApiUrlType::RolesGetUsersInRole => "roles.getUsersInRole".to_string(),
            RestApiUrlType::RolesAddUserToRole => "roles.addUserToRole".to_string(),
            RestApiUrlType::RolesRemoveUserFromRole => "roles.removeUserFromRole".to_string(),

            RestApiUrlType::RolesDelete => "roles.delete".to_string(),

            RestApiUrlType::VideoConfJitsi => "video-conference/jitsi.update-timeout".to_string(),

            RestApiUrlType::AutoTranslateGetSupportedLanguages => {
                "autotranslate.getSupportedLanguages".to_string()
            }
            RestApiUrlType::AutoTranslateSaveSettings => "autotranslate.saveSettings".to_string(),
            RestApiUrlType::AutoTranslateTranslateMessage => {
                "autotranslate.translateMessage".to_string()
            }

            RestApiUrlType::CustomUserStatusList => "custom-user-status.list".to_string(),
            RestApiUrlType::CustomUserStatusCreate => "custom-user-status.create".to_string(),
            RestApiUrlType::CustomUserStatusDelete => "custom-user-status.delete".to_string(),
            RestApiUrlType::CustomUserStatusUpdate => "custom-user-status.update".to_string(),

            RestApiUrlType::CustomSoundsList => "custom-sounds.list".to_string(),
            RestApiUrlType::FindOrCreateInvite => "findOrCreateInvite".to_string(),
            RestApiUrlType::ListInvites => "listInvites".to_string(),
            RestApiUrlType::RemoveInvite => "removeInvite".to_string(),
            RestApiUrlType::UseInviteToken => "useInviteToken".to_string(),
            RestApiUrlType::ValidateInviteToken => "validateInviteToken".to_string(),
            RestApiUrlType::SendInvitationEmails => "sendInvitationEmail".to_string(),
            RestApiUrlType::RoomsAutocompleteChannelAndPrivate => {
                "rooms.autocomplete.channelAndPrivate".to_string()
            }
            RestApiUrlType::Users2FASendEmailCode => "users.2fa.sendEmailCode".to_string(),
            RestApiUrlType::Users2FAEnableEmail => "users.2fa.enableEmail".to_string(),
            RestApiUrlType::Users2FADisableEmail => "users.2fa.disableEmail".to_string(),

            RestApiUrlType::UsersDeleteOwnAccount => "users.deleteOwnAccount".to_string(),
            RestApiUrlType::UsersSetPreferences => "users.setPreferences".to_string(),
            RestApiUrlType::PermissionsListAll => "permissions.listAll".to_string(),
            RestApiUrlType::PermissionsUpdate => "permissions.update".to_string(),
            RestApiUrlType::Statistics => "statistics".to_string(),
            RestApiUrlType::Directory => "directory".to_string(),

            RestApiUrlType::TeamsList => "teams.list".to_string(),
            RestApiUrlType::TeamsListAll => "teams.listAll".to_string(),
            RestApiUrlType::TeamsCreate => "teams.create".to_string(),
            RestApiUrlType::TeamsAddRooms => "teams.addRooms".to_string(),
            RestApiUrlType::TeamsRemoveRoom => "teams.removeRoom".to_string(),
            RestApiUrlType::TeamsUpdateRoom => "teams.updateRoom".to_string(),
            RestApiUrlType::TeamsListRooms => "teams.listRooms".to_string(),
            RestApiUrlType::TeamsListRoomsOfUser => "teams.listRoomsOfUser".to_string(),
            RestApiUrlType::TeamsMembers => "teams.members".to_string(),
            RestApiUrlType::TeamsAddMembers => "teams.addMembers".to_string(),
            RestApiUrlType::TeamsUpdateMember => "teams.updateMember".to_string(),
            RestApiUrlType::TeamsRemoveMember => "teams.removeMember".to_string(),
            RestApiUrlType::TeamsLeave => "teams.leave".to_string(),
            RestApiUrlType::TeamsInfo => "teams.info".to_string(),
            RestApiUrlType::TeamsDelete => "teams.delete".to_string(),
            RestApiUrlType::TeamsAutocomplete => "teams.autocomplete".to_string(),
            RestApiUrlType::TeamsConvertToChannel => "teams.convertToChannel".to_string(),
            RestApiUrlType::RoomsAutocompleteAvailableForTeams => {
                "rooms.autocomplete.availableForTeams".to_string()
            }
            RestApiUrlType::ChannelsConvertToTeam => "channels.convertToTeam".to_string(),
            RestApiUrlType::GroupsConvertToTeam => "groups.convertToTeam".to_string(),
            RestApiUrlType::StdoutQueue => "stdout.queue".to_string(),
            RestApiUrlType::OauthAppsList => "oauth-apps.list".to_string(),
            RestApiUrlType::OauthAppsGet => "oauth-apps.get".to_string(),
            RestApiUrlType::OauthAppsCreate => "oauth-apps.create".to_string(),
            RestApiUrlType::OauthAppsUpdate => "oauth-apps.update".to_string(),
            RestApiUrlType::OauthAppsDelete => "oauth-apps.delete".to_string(),

            RestApiUrlType::SettingsOauth => "settings.oauth".to_string(),
            RestApiUrlType::SettingsAddCustomOauth => "settings.addCustomOAuth".to_string(),

            RestApiUrlType::LicensesGet => "licenses.get".to_string(),
            RestApiUrlType::LicensesMaxActiveUsers => "licenses.maxActiveUsers".to_string(),
            RestApiUrlType::LicensesAdd => "licenses.add".to_string(),
            RestApiUrlType::LicensesIsEntreprise => "licenses.isEnterprise".to_string(),
            RestApiUrlType::LicensesInfo => "licenses.info".to_string(),

            RestApiUrlType::BannersDismiss => "banners.dismiss".to_string(),
            RestApiUrlType::Banners => "banners".to_string(),

            // RC 5.0
            RestApiUrlType::SessionsList => "sessions/list".to_string(),
            RestApiUrlType::SessionsInfo => "sessions/info".to_string(),
            RestApiUrlType::SessionsListAll => "sessions/list.all".to_string(),
            RestApiUrlType::SessionsLogoutMe => "sessions/logout.me".to_string(),
            RestApiUrlType::SessionsLogout => "sessions/logout".to_string(),
            RestApiUrlType::SessionsInfoAdmin => "sessions/info.admin".to_string(),

            RestApiUrlType::UserRequestDataDownload => "users.requestDataDownload".to_string(),

            RestApiUrlType::VideoConferenceStart => "video-conference.start".to_string(),
            RestApiUrlType::VideoConferenceJoin => "video-conference.join".to_string(),
            RestApiUrlType::VideoConferenceCancel => "video-conference.cancel".to_string(),
            RestApiUrlType::VideoConferenceInfo => "video-conference.info".to_string(),
            RestApiUrlType::VideoConferenceList => "video-conference.list".to_string(),
            RestApiUrlType::VideoConferenceProviders => "video-conference.providers".to_string(),
            RestApiUrlType::VideoConferenceCapabilities => {
                "video-conference.capabilities".to_string()
            }
            RestApiUrlType::ModerationReportsByUsers => "moderation.reportsByUsers".to_string(),
            RestApiUrlType::ModerationDismissReports => "moderation.dismissReports".to_string(),
            RestApiUrlType::ModerationUserReportedMessages => {
                "moderation.user.reportedMessages".to_string()
            }
            RestApiUrlType::ModerationUserDeleteReportedMessages => {
                "moderation.user.deleteReportedMessages".to_string()
            }
            RestApiUrlType::ModerationReports => "moderation.reports".to_string(),
            RestApiUrlType::ModerationReportInfo => "moderation.reportInfo".to_string(),
            RestApiUrlType::ModerationUserReports => "moderation.userReports".to_string(),
            RestApiUrlType::ModerationUserReportsByUserId => {
                "moderation.user.reportsByUserId".to_string()
            }
            RestApiUrlType::ModerationDismissUserReports => {
                "moderation.dismissUserReports".to_string()
            }
            RestApiUrlType::ModerationReportUser => "moderation.reportUser".to_string(),

            RestApiUrlType::RoomsNameExists => "rooms.nameExists".to_string(),
            RestApiUrlType::MethodCall => "method.call".to_string(),
            RestApiUrlType::MethodCallAnon => "method.callAnon".to_string(),
            RestApiUrlType::AppsUiInteraction => "ui.interaction".to_string(),

            RestApiUrlType::UsersLogoutOtherClients => "users.logoutOtherClients".to_string(),

            // 6.8.0
            RestApiUrlType::RoomsMuteUser => "rooms.muteUser".to_string(),
            RestApiUrlType::RoomsUnmuteUser => "rooms.unmuteUser".to_string(),

            RestApiUrlType::FeaturedApps => "featured-apps".to_string(),
            RestApiUrlType::CategoriesApps => "categories".to_string(),
            RestApiUrlType::CountApps => "count".to_string(),
            RestApiUrlType::NotifyAdminsApps => "notify-admins".to_string(),
            RestApiUrlType::InstalledApps => "installed".to_string(),
            RestApiUrlType::MarketplaceApps => "marketplace".to_string(),

            RestApiUrlType::UsersListByStatus => "users.listByStatus".to_string(),

            RestApiUrlType::UsersSendWelcomeEmail => "users.sendWelcomeEmail".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::methods::restapiutils::RestApiUrlExtensionType;
    use crate::methods::restapiutils::RestApiUrlType;

    #[test]
    fn test_verify_enum_restapiurlextensiontype() {
        assert_eq!(RestApiUrlExtensionType::V1.path(), "/api/v1/");
        assert_eq!(RestApiUrlExtensionType::NoExtension.path(), "/api/");
        assert_eq!(RestApiUrlExtensionType::Apps.path(), "/api/apps/");
    }
    #[test]
    fn test_verify_enum_restapiurltype() {
        assert_eq!(RestApiUrlType::BannersDismiss.path(), "banners.dismiss");
        assert_eq!(RestApiUrlType::Login.path(), "login");
        assert_eq!(RestApiUrlType::Logout.path(), "logout");
        assert_eq!(RestApiUrlType::Me.path(), "me");
        assert_eq!(RestApiUrlType::UsersGetAvatar.path(), "users.getAvatar");
        assert_eq!(RestApiUrlType::UsersDelete.path(), "users.delete");
        assert_eq!(RestApiUrlType::UsersCreate.path(), "users.create");
        assert_eq!(RestApiUrlType::UsersCreateToken.path(), "users.createToken");
        assert_eq!(RestApiUrlType::UsersGetPresence.path(), "users.getPresence");
        assert_eq!(RestApiUrlType::UsersInfo.path(), "users.info");
        assert_eq!(RestApiUrlType::UsersList.path(), "users.list");
        assert_eq!(RestApiUrlType::UsersRegister.path(), "users.register");
        assert_eq!(RestApiUrlType::UsersResetAvatar.path(), "users.resetAvatar");
        assert_eq!(RestApiUrlType::UsersSetAvatar.path(), "users.setAvatar");
        assert_eq!(RestApiUrlType::UsersUpdate.path(), "users.update");
        assert_eq!(
            RestApiUrlType::UsersGetUsernameSuggestion.path(),
            "users.getUsernameSuggestion"
        );

        assert_eq!(
            RestApiUrlType::UsersRemovePersonalAccessToken.path(),
            "users.removePersonalAccessToken"
        );
        assert_eq!(
            RestApiUrlType::UsersGeneratePersonalAccessToken.path(),
            "users.generatePersonalAccessToken"
        );
        assert_eq!(
            RestApiUrlType::UsersRegeneratePersonalAccessToken.path(),
            "users.regeneratePersonalAccessToken"
        );
        assert_eq!(
            RestApiUrlType::UsersGetPersonalAccessTokens.path(),
            "users.getPersonalAccessTokens"
        );
        assert_eq!(RestApiUrlType::UsersPresence.path(), "users.presence");
        assert_eq!(
            RestApiUrlType::UsersUpdateOwnBasicInfo.path(),
            "users.updateOwnBasicInfo"
        );
        assert_eq!(RestApiUrlType::UsersSetStatus.path(), "users.setStatus");
        assert_eq!(RestApiUrlType::UsersGetStatus.path(), "users.getStatus");
        assert_eq!(
            RestApiUrlType::UsersAutocomplete.path(),
            "users.autocomplete"
        );
        assert_eq!(
            RestApiUrlType::UsersRemoveOtherTokens.path(),
            "users.removeOtherTokens"
        );
        assert_eq!(
            RestApiUrlType::UsersSetActiveStatus.path(),
            "users.setActiveStatus"
        );
        assert_eq!(RestApiUrlType::UsersResetTOTP.path(), "users.resetTOTP");
        assert_eq!(RestApiUrlType::UsersResetE2EKey.path(), "users.resetE2EKey");

        assert_eq!(RestApiUrlType::ChatDelete.path(), "chat.delete");
        assert_eq!(RestApiUrlType::ChatGetMessage.path(), "chat.getMessage");
        assert_eq!(RestApiUrlType::ChatPinMessage.path(), "chat.pinMessage");
        assert_eq!(RestApiUrlType::ChatPostMessage.path(), "chat.postMessage");
        assert_eq!(RestApiUrlType::ChatReact.path(), "chat.react");
        assert_eq!(RestApiUrlType::ChatStarMessage.path(), "chat.starMessage");
        assert_eq!(RestApiUrlType::ChatUnPinMessage.path(), "chat.unPinMessage");
        assert_eq!(
            RestApiUrlType::ChatUnStarMessage.path(),
            "chat.unStarMessage"
        );
        assert_eq!(RestApiUrlType::ChatUpdate.path(), "chat.update");
        assert_eq!(RestApiUrlType::ChatSearch.path(), "chat.search");
        assert_eq!(RestApiUrlType::ChatIgnoreUser.path(), "chat.ignoreUser");
        assert_eq!(
            RestApiUrlType::ChatReportMessage.path(),
            "chat.reportMessage"
        );
        assert_eq!(
            RestApiUrlType::ChatFollowMessage.path(),
            "chat.followMessage"
        );
        assert_eq!(
            RestApiUrlType::ChatUnFollowMessage.path(),
            "chat.unfollowMessage"
        );
        assert_eq!(
            RestApiUrlType::ChatGetDeletedMessage.path(),
            "chat.getDeletedMessages"
        );
        assert_eq!(
            RestApiUrlType::ChatMessageReadReceipts.path(),
            "chat.getMessageReadReceipts"
        );
        assert_eq!(
            RestApiUrlType::ChatSyncThreadsList.path(),
            "chat.syncThreadsList"
        );
        assert_eq!(
            RestApiUrlType::ChatGetThreadsList.path(),
            "chat.getThreadsList"
        );
        assert_eq!(
            RestApiUrlType::ChatSyncThreadMessages.path(),
            "chat.syncThreadMessages"
        );
        assert_eq!(
            RestApiUrlType::ChatGetThreadMessages.path(),
            "chat.getThreadMessages"
        );
        assert_eq!(RestApiUrlType::ChatSendMessage.path(), "chat.sendMessage");
        assert_eq!(
            RestApiUrlType::ChatGetPinnedMessages.path(),
            "chat.getPinnedMessages"
        );
        assert_eq!(
            RestApiUrlType::ChatGetMentionedMessages.path(),
            "chat.getMentionedMessages"
        );
        assert_eq!(
            RestApiUrlType::ChatGetStarredMessages.path(),
            "chat.getStarredMessages"
        );
        assert_eq!(
            RestApiUrlType::ChatGetSnippetedMessages.path(),
            "chat.getSnippetedMessages"
        );
        assert_eq!(RestApiUrlType::ChatSyncMessages.path(), "chat.syncMessages");
        assert_eq!(RestApiUrlType::ChannelsAddAll.path(), "channels.addAll");
        assert_eq!(
            RestApiUrlType::ChannelsAddModerator.path(),
            "channels.addModerator"
        );
        assert_eq!(RestApiUrlType::ChannelsAddOwner.path(), "channels.addOwner");
        assert_eq!(
            RestApiUrlType::ChannelsAddLeader.path(),
            "channels.addLeader"
        );
        assert_eq!(RestApiUrlType::ChannelsArchive.path(), "channels.archive");
        assert_eq!(
            RestApiUrlType::ChannelsModerators.path(),
            "channels.moderators"
        );
        assert_eq!(RestApiUrlType::ChannelsClose.path(), "channels.close");
        assert_eq!(RestApiUrlType::ChannelsCreate.path(), "channels.create");
        assert_eq!(
            RestApiUrlType::ChannelsGetIntegrations.path(),
            "channels.getIntegrations"
        );
        assert_eq!(RestApiUrlType::ChannelsHistory.path(), "channels.history");
        assert_eq!(RestApiUrlType::ChannelsInfo.path(), "channels.info");
        assert_eq!(RestApiUrlType::ChannelsInvite.path(), "channels.invite");
        assert_eq!(RestApiUrlType::ChannelsKick.path(), "channels.kick");
        assert_eq!(RestApiUrlType::ChannelsLeave.path(), "channels.leave");
        assert_eq!(RestApiUrlType::ChannelsList.path(), "channels.list");
        assert_eq!(
            RestApiUrlType::ChannelsListJoined.path(),
            "channels.list.joined"
        );
        assert_eq!(RestApiUrlType::ChannelsOpen.path(), "channels.open");
        assert_eq!(
            RestApiUrlType::ChannelsRemoveModerator.path(),
            "channels.removeModerator"
        );
        assert_eq!(
            RestApiUrlType::ChannelsRemoveOwner.path(),
            "channels.removeOwner"
        );
        assert_eq!(
            RestApiUrlType::ChannelsRemoveLeader.path(),
            "channels.removeLeader"
        );
        assert_eq!(RestApiUrlType::ChannelsRename.path(), "channels.rename");
        assert_eq!(
            RestApiUrlType::ChannelsSetDescription.path(),
            "channels.setDescription"
        );
        assert_eq!(
            RestApiUrlType::ChannelsSetJoinCode.path(),
            "channels.setJoinCode"
        );
        assert_eq!(
            RestApiUrlType::ChannelsSetPurpose.path(),
            "channels.setPurpose"
        );
        assert_eq!(
            RestApiUrlType::ChannelsSetReadOnly.path(),
            "channels.setReadOnly"
        );
        assert_eq!(RestApiUrlType::ChannelsSetTopic.path(), "channels.setTopic");
        assert_eq!(
            RestApiUrlType::ChannelsSetAnnouncement.path(),
            "channels.setAnnouncement"
        );
        assert_eq!(RestApiUrlType::ChannelsFiles.path(), "channels.files");
        assert_eq!(RestApiUrlType::ChannelsSetType.path(), "channels.setType");
        assert_eq!(
            RestApiUrlType::ChannelsUnarchive.path(),
            "channels.unarchive"
        );
        assert_eq!(RestApiUrlType::ChannelsRoles.path(), "channels.roles");
        assert_eq!(RestApiUrlType::ChannelsCounters.path(), "channels.counters");
        assert_eq!(RestApiUrlType::ChannelsJoin.path(), "channels.join");
        assert_eq!(RestApiUrlType::ChannelsMembers.path(), "channels.members");
        assert_eq!(RestApiUrlType::ChannelsDelete.path(), "channels.delete");
        assert_eq!(RestApiUrlType::ChannelsOnline.path(), "channels.online");
        assert_eq!(
            RestApiUrlType::ChannelsGetAllUserMentionsByChannel.path(),
            "channels.getAllUserMentionsByChannel"
        );
        assert_eq!(RestApiUrlType::GroupsAddAll.path(), "groups.addAll");
        assert_eq!(
            RestApiUrlType::GroupsAddModerator.path(),
            "groups.addModerator"
        );
        assert_eq!(RestApiUrlType::GroupsAddOwner.path(), "groups.addOwner");
        assert_eq!(RestApiUrlType::GroupsArchive.path(), "groups.archive");
        assert_eq!(RestApiUrlType::GroupsClose.path(), "groups.close");
        assert_eq!(RestApiUrlType::GroupsCreate.path(), "groups.create");
        assert_eq!(
            RestApiUrlType::GroupsGetIntegrations.path(),
            "groups.getIntegrations"
        );
        assert_eq!(RestApiUrlType::GroupsHistory.path(), "groups.history");
        assert_eq!(RestApiUrlType::GroupsInfo.path(), "groups.info");
        assert_eq!(RestApiUrlType::GroupsInvite.path(), "groups.invite");
        assert_eq!(RestApiUrlType::GroupsKick.path(), "groups.kick");
        assert_eq!(RestApiUrlType::GroupsLeave.path(), "groups.leave");
        assert_eq!(RestApiUrlType::GroupsList.path(), "groups.list");
        assert_eq!(RestApiUrlType::GroupsOpen.path(), "groups.open");
        assert_eq!(
            RestApiUrlType::GroupsRemoveModerator.path(),
            "groups.removeModerator"
        );
        assert_eq!(
            RestApiUrlType::GroupsRemoveOwner.path(),
            "groups.removeOwner"
        );
        assert_eq!(RestApiUrlType::GroupsRename.path(), "groups.rename");
        assert_eq!(
            RestApiUrlType::GroupsSetDescription.path(),
            "groups.setDescription"
        );
        assert_eq!(RestApiUrlType::GroupsSetPurpose.path(), "groups.setPurpose");
        assert_eq!(
            RestApiUrlType::GroupsSetReadOnly.path(),
            "groups.setReadOnly"
        );
        assert_eq!(RestApiUrlType::GroupsSetTopic.path(), "groups.setTopic");
        assert_eq!(RestApiUrlType::GroupsSetType.path(), "groups.setType");
        assert_eq!(RestApiUrlType::GroupsUnarchive.path(), "groups.unarchive");
        assert_eq!(
            RestApiUrlType::GroupsSetAnnouncement.path(),
            "groups.setAnnouncement"
        );
        assert_eq!(RestApiUrlType::GroupsFiles.path(), "groups.files");
        assert_eq!(RestApiUrlType::GroupsRoles.path(), "groups.roles");
        assert_eq!(RestApiUrlType::GroupsCounters.path(), "groups.counters");
        assert_eq!(
            RestApiUrlType::GroupsRemoveLeader.path(),
            "groups.removeLeader"
        );
        assert_eq!(RestApiUrlType::GroupsAddLeader.path(), "groups.addLeader");
        assert_eq!(RestApiUrlType::GroupsDelete.path(), "groups.delete");
        assert_eq!(RestApiUrlType::GroupsListAll.path(), "groups.listAll");
        assert_eq!(RestApiUrlType::GroupsMembers.path(), "groups.members");
        assert_eq!(
            RestApiUrlType::GroupsSetEncrypted.path(),
            "groups.setEncrypted"
        );
        assert_eq!(RestApiUrlType::ServerInfo.path(), "info");
        assert_eq!(RestApiUrlType::Settings.path(), "settings");
        assert_eq!(RestApiUrlType::SettingsPublic.path(), "settings.public");
        assert_eq!(RestApiUrlType::UpdateAdminSettings.path(), "settings");
        assert_eq!(RestApiUrlType::RoomsUpload.path(), "rooms.upload");
        assert_eq!(
            RestApiUrlType::RoomsSaveNotification.path(),
            "rooms.saveNotification"
        );
        assert_eq!(
            RestApiUrlType::RoomsSaveSettings.path(),
            "rooms.saveRoomSettings"
        );
        assert_eq!(RestApiUrlType::RoomsAdminRooms.path(), "rooms.adminRooms");
        assert_eq!(
            RestApiUrlType::RoomsAdminRoomsGetRoom.path(),
            "rooms.adminRooms.getRoom"
        );
        assert_eq!(RestApiUrlType::Spotlight.path(), "spotlight");
        assert_eq!(RestApiUrlType::ImClose.path(), "im.close");
        assert_eq!(RestApiUrlType::ImCreate.path(), "im.create");
        assert_eq!(RestApiUrlType::ImOpen.path(), "im.open");
        assert_eq!(RestApiUrlType::ImSetTopic.path(), "im.setTopic");
        assert_eq!(RestApiUrlType::ImHistory.path(), "im.history");
        assert_eq!(RestApiUrlType::ImFiles.path(), "im.files");
        assert_eq!(RestApiUrlType::ImDelete.path(), "im.delete");
        assert_eq!(RestApiUrlType::ImMessages.path(), "im.messages");
        assert_eq!(RestApiUrlType::ImMembers.path(), "im.members");
        assert_eq!(RestApiUrlType::LoadEmojiCustom.path(), "emoji-custom.list");
        assert_eq!(
            RestApiUrlType::EmojiCustomDelete.path(),
            "emoji-custom.delete"
        );
        assert_eq!(
            RestApiUrlType::EmojiCustomCreate.path(),
            "emoji-custom.create"
        );
        assert_eq!(
            RestApiUrlType::EmojiCustomUpdate.path(),
            "emoji-custom.update"
        );
        assert_eq!(RestApiUrlType::EmojiCustomAll.path(), "emoji-custom.all");
        assert_eq!(
            RestApiUrlType::SubscriptionsRead.path(),
            "subscriptions.read"
        );
        assert_eq!(
            RestApiUrlType::SubscriptionsUnRead.path(),
            "subscriptions.unread"
        );
        assert_eq!(RestApiUrlType::RoomsGet.path(), "rooms.get");
        assert_eq!(RestApiUrlType::RoomsFavorite.path(), "rooms.favorite");
        assert_eq!(
            RestApiUrlType::RoomsCleanHistory.path(),
            "rooms.cleanHistory"
        );
        // since 1.0.0
        assert_eq!(
            RestApiUrlType::RoomsCreateDiscussion.path(),
            "rooms.createDiscussion"
        );
        assert_eq!(
            RestApiUrlType::RoomsGetDiscussions.path(),
            "rooms.getDiscussions"
        );

        assert_eq!(RestApiUrlType::RoomsExport.path(), "rooms.export");
        assert_eq!(
            RestApiUrlType::RoomsChangeArchivactionState.path(),
            "rooms.changeArchivationState"
        );
        assert_eq!(RestApiUrlType::RoomsInfo.path(), "rooms.info");
        assert_eq!(RestApiUrlType::RoomsLeave.path(), "rooms.leave");
        // Since 5.4.0
        assert_eq!(RestApiUrlType::RoomsDelete.path(), "rooms.delete");

        //
        assert_eq!(
            RestApiUrlType::ForgotPassword.path(),
            "users.forgotPassword"
        );
        assert_eq!(RestApiUrlType::CommandsList.path(), "commands.list");
        assert_eq!(RestApiUrlType::CommandsGet.path(), "commands.get");
        assert_eq!(RestApiUrlType::CommandsRun.path(), "commands.run");
        assert_eq!(RestApiUrlType::CommandsPreview.path(), "commands.preview");
        assert_eq!(RestApiUrlType::E2EfetchMyKeys.path(), "e2e.fetchMyKeys");
        assert_eq!(
            RestApiUrlType::E2EupdateGroupKey.path(),
            "e2e.updateGroupKey"
        );

        assert_eq!(RestApiUrlType::E2ESetRoomKeyID.path(), "e2e.setRoomKeyID");
        assert_eq!(
            RestApiUrlType::E2ESetUserPublicAndPrivateKeys.path(),
            "e2e.setUserPublicAndPrivateKeys"
        );
        assert_eq!(
            RestApiUrlType::E2EGetUsersOfRoomWithoutKey.path(),
            "e2e.getUsersOfRoomWithoutKey"
        );
        assert_eq!(
            RestApiUrlType::E2ERequestSubscriptionKeys.path(),
            "e2e.requestSubscriptionKeys"
        );
        assert_eq!(
            RestApiUrlType::E2EResetOwnE2EKey.path(),
            "e2e.resetOwnE2EKey"
        );

        assert_eq!(RestApiUrlType::RolesList.path(), "roles.list");
        assert_eq!(RestApiUrlType::RolesCreate.path(), "roles.create");
        assert_eq!(RestApiUrlType::RolesSync.path(), "roles.sync");
        assert_eq!(RestApiUrlType::RolesUpdate.path(), "roles.update");
        assert_eq!(
            RestApiUrlType::RolesGetUsersInRole.path(),
            "roles.getUsersInRole"
        );
        assert_eq!(
            RestApiUrlType::RolesAddUserToRole.path(),
            "roles.addUserToRole"
        );
        assert_eq!(
            RestApiUrlType::RolesRemoveUserFromRole.path(),
            "roles.removeUserFromRole"
        );

        assert_eq!(RestApiUrlType::RolesDelete.path(), "roles.delete");

        assert_eq!(
            RestApiUrlType::VideoConfJitsi.path(),
            "video-conference/jitsi.update-timeout"
        );

        assert_eq!(
            RestApiUrlType::AutoTranslateGetSupportedLanguages.path(),
            "autotranslate.getSupportedLanguages"
        );
        assert_eq!(
            RestApiUrlType::AutoTranslateSaveSettings.path(),
            "autotranslate.saveSettings"
        );
        assert_eq!(
            RestApiUrlType::AutoTranslateTranslateMessage.path(),
            "autotranslate.translateMessage"
        );

        assert_eq!(
            RestApiUrlType::CustomUserStatusList.path(),
            "custom-user-status.list"
        );
        assert_eq!(
            RestApiUrlType::CustomUserStatusCreate.path(),
            "custom-user-status.create"
        );
        assert_eq!(
            RestApiUrlType::CustomUserStatusDelete.path(),
            "custom-user-status.delete"
        );
        assert_eq!(
            RestApiUrlType::CustomUserStatusUpdate.path(),
            "custom-user-status.update"
        );

        assert_eq!(
            RestApiUrlType::CustomSoundsList.path(),
            "custom-sounds.list"
        );
        assert_eq!(
            RestApiUrlType::FindOrCreateInvite.path(),
            "findOrCreateInvite"
        );
        assert_eq!(RestApiUrlType::ListInvites.path(), "listInvites");
        assert_eq!(RestApiUrlType::RemoveInvite.path(), "removeInvite");
        assert_eq!(RestApiUrlType::UseInviteToken.path(), "useInviteToken");
        assert_eq!(
            RestApiUrlType::ValidateInviteToken.path(),
            "validateInviteToken"
        );
        assert_eq!(
            RestApiUrlType::SendInvitationEmails.path(),
            "sendInvitationEmail"
        );
        assert_eq!(
            RestApiUrlType::RoomsAutocompleteChannelAndPrivate.path(),
            "rooms.autocomplete.channelAndPrivate"
        );

        assert_eq!(
            RestApiUrlType::Users2FASendEmailCode.path(),
            "users.2fa.sendEmailCode"
        );
        assert_eq!(
            RestApiUrlType::Users2FAEnableEmail.path(),
            "users.2fa.enableEmail"
        );
        assert_eq!(
            RestApiUrlType::Users2FADisableEmail.path(),
            "users.2fa.disableEmail"
        );

        assert_eq!(
            RestApiUrlType::UsersDeleteOwnAccount.path(),
            "users.deleteOwnAccount"
        );
        assert_eq!(
            RestApiUrlType::UsersSetPreferences.path(),
            "users.setPreferences"
        );
        assert_eq!(
            RestApiUrlType::PermissionsListAll.path(),
            "permissions.listAll"
        );
        assert_eq!(
            RestApiUrlType::PermissionsUpdate.path(),
            "permissions.update"
        );
        assert_eq!(RestApiUrlType::Statistics.path(), "statistics");
        assert_eq!(RestApiUrlType::Directory.path(), "directory");

        assert_eq!(RestApiUrlType::TeamsList.path(), "teams.list");
        assert_eq!(RestApiUrlType::TeamsListAll.path(), "teams.listAll");
        assert_eq!(RestApiUrlType::TeamsCreate.path(), "teams.create");
        assert_eq!(RestApiUrlType::TeamsAddRooms.path(), "teams.addRooms");
        assert_eq!(RestApiUrlType::TeamsRemoveRoom.path(), "teams.removeRoom");
        assert_eq!(RestApiUrlType::TeamsUpdateRoom.path(), "teams.updateRoom");
        assert_eq!(RestApiUrlType::TeamsListRooms.path(), "teams.listRooms");
        assert_eq!(
            RestApiUrlType::TeamsListRoomsOfUser.path(),
            "teams.listRoomsOfUser"
        );
        assert_eq!(RestApiUrlType::TeamsMembers.path(), "teams.members");
        assert_eq!(RestApiUrlType::TeamsAddMembers.path(), "teams.addMembers");
        assert_eq!(
            RestApiUrlType::TeamsUpdateMember.path(),
            "teams.updateMember"
        );
        assert_eq!(
            RestApiUrlType::TeamsRemoveMember.path(),
            "teams.removeMember"
        );
        assert_eq!(RestApiUrlType::TeamsLeave.path(), "teams.leave");
        assert_eq!(RestApiUrlType::TeamsInfo.path(), "teams.info");
        assert_eq!(RestApiUrlType::TeamsDelete.path(), "teams.delete");
        assert_eq!(
            RestApiUrlType::TeamsAutocomplete.path(),
            "teams.autocomplete"
        );
        assert_eq!(
            RestApiUrlType::TeamsConvertToChannel.path(),
            "teams.convertToChannel"
        );
        assert_eq!(
            RestApiUrlType::RoomsAutocompleteAvailableForTeams.path(),
            "rooms.autocomplete.availableForTeams"
        );
        assert_eq!(
            RestApiUrlType::ChannelsConvertToTeam.path(),
            "channels.convertToTeam"
        );
        assert_eq!(
            RestApiUrlType::GroupsConvertToTeam.path(),
            "groups.convertToTeam"
        );
        assert_eq!(RestApiUrlType::StdoutQueue.path(), "stdout.queue");
        assert_eq!(RestApiUrlType::OauthAppsList.path(), "oauth-apps.list");
        assert_eq!(RestApiUrlType::OauthAppsGet.path(), "oauth-apps.get");
        assert_eq!(RestApiUrlType::OauthAppsCreate.path(), "oauth-apps.create");
        assert_eq!(RestApiUrlType::OauthAppsUpdate.path(), "oauth-apps.update");
        assert_eq!(RestApiUrlType::OauthAppsDelete.path(), "oauth-apps.delete");

        assert_eq!(RestApiUrlType::SettingsOauth.path(), "settings.oauth");
        assert_eq!(
            RestApiUrlType::SettingsAddCustomOauth.path(),
            "settings.addCustomOAuth"
        );

        assert_eq!(RestApiUrlType::LicensesGet.path(), "licenses.get");
        assert_eq!(
            RestApiUrlType::LicensesMaxActiveUsers.path(),
            "licenses.maxActiveUsers"
        );
        assert_eq!(RestApiUrlType::LicensesAdd.path(), "licenses.add");
        assert_eq!(
            RestApiUrlType::LicensesIsEntreprise.path(),
            "licenses.isEnterprise"
        );
        assert_eq!(RestApiUrlType::LicensesInfo.path(), "licenses.info");

        assert_eq!(RestApiUrlType::BannersDismiss.path(), "banners.dismiss");
        assert_eq!(RestApiUrlType::Banners.path(), "banners");

        assert_eq!(RestApiUrlType::SessionsList.path(), "sessions/list");
        assert_eq!(RestApiUrlType::SessionsInfo.path(), "sessions/info");
        assert_eq!(RestApiUrlType::SessionsListAll.path(), "sessions/list.all");
        assert_eq!(
            RestApiUrlType::SessionsLogoutMe.path(),
            "sessions/logout.me"
        );
        assert_eq!(RestApiUrlType::SessionsLogout.path(), "sessions/logout");
        assert_eq!(
            RestApiUrlType::SessionsInfoAdmin.path(),
            "sessions/info.admin"
        );

        assert_eq!(
            RestApiUrlType::UserRequestDataDownload.path(),
            "users.requestDataDownload"
        );

        assert_eq!(
            RestApiUrlType::VideoConferenceStart.path(),
            "video-conference.start"
        );
        assert_eq!(
            RestApiUrlType::VideoConferenceJoin.path(),
            "video-conference.join"
        );
        assert_eq!(
            RestApiUrlType::VideoConferenceCancel.path(),
            "video-conference.cancel"
        );
        assert_eq!(
            RestApiUrlType::VideoConferenceInfo.path(),
            "video-conference.info"
        );
        assert_eq!(
            RestApiUrlType::VideoConferenceList.path(),
            "video-conference.list"
        );
        assert_eq!(
            RestApiUrlType::VideoConferenceProviders.path(),
            "video-conference.providers"
        );
        assert_eq!(
            RestApiUrlType::VideoConferenceCapabilities.path(),
            "video-conference.capabilities"
        );
        assert_eq!(
            RestApiUrlType::ModerationReportsByUsers.path(),
            "moderation.reportsByUsers"
        );
        assert_eq!(
            RestApiUrlType::ModerationDismissReports.path(),
            "moderation.dismissReports"
        );
        assert_eq!(
            RestApiUrlType::ModerationUserReportedMessages.path(),
            "moderation.user.reportedMessages"
        );
        assert_eq!(
            RestApiUrlType::ModerationUserDeleteReportedMessages.path(),
            "moderation.user.deleteReportedMessages"
        );
        assert_eq!(
            RestApiUrlType::ModerationReports.path(),
            "moderation.reports"
        );
        assert_eq!(
            RestApiUrlType::ModerationReportInfo.path(),
            "moderation.reportInfo"
        );
        assert_eq!(
            RestApiUrlType::ModerationUserReports.path(),
            "moderation.userReports"
        );
        assert_eq!(
            RestApiUrlType::ModerationUserReportsByUserId.path(),
            "moderation.user.reportsByUserId"
        );
        assert_eq!(
            RestApiUrlType::ModerationDismissUserReports.path(),
            "moderation.dismissUserReports"
        );
        assert_eq!(
            RestApiUrlType::ModerationReportUser.path(),
            "moderation.reportUser"
        );

        assert_eq!(RestApiUrlType::RoomsNameExists.path(), "rooms.nameExists");
        assert_eq!(RestApiUrlType::MethodCall.path(), "method.call");
        assert_eq!(RestApiUrlType::MethodCallAnon.path(), "method.callAnon");
        assert_eq!(RestApiUrlType::AppsUiInteraction.path(), "ui.interaction");

        assert_eq!(
            RestApiUrlType::UsersLogoutOtherClients.path(),
            "users.logoutOtherClients"
        );

        // 6.8.0
        assert_eq!(RestApiUrlType::RoomsMuteUser.path(), "rooms.muteUser");
        assert_eq!(RestApiUrlType::RoomsUnmuteUser.path(), "rooms.unmuteUser");

        assert_eq!(RestApiUrlType::FeaturedApps.path(), "featured-apps");
        assert_eq!(RestApiUrlType::CategoriesApps.path(), "categories");
        assert_eq!(RestApiUrlType::CountApps.path(), "count");
        assert_eq!(RestApiUrlType::NotifyAdminsApps.path(), "notify-admins");
        assert_eq!(RestApiUrlType::InstalledApps.path(), "installed");
        assert_eq!(RestApiUrlType::MarketplaceApps.path(), "marketplace");

        assert_eq!(
            RestApiUrlType::UsersListByStatus.path(),
            "users.listByStatus"
        );

        assert_eq!(
            RestApiUrlType::UsersSendWelcomeEmail.path(),
            "users.sendWelcomeEmail"
        );
    }
}
