/*
 * SPDX-FileCopyrightText: 2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
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
