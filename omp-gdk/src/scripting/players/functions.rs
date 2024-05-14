use super::{
    MapIconStyle, Player, PlayerAimData, PlayerAnimationData, PlayerAnimationSyncType,
    PlayerBulletData, PlayerCameraCutType, PlayerFightingStyle, PlayerKeyData, PlayerNameStatus,
    PlayerSpecialAction, PlayerSpectateData, PlayerSpectateMode, PlayerState, PlayerSurfingData,
    PlayerWeapon, PlayerWeaponSkill, WeaponSlotData,
};
use omp_codegen::native;
use std::ffi::c_void;

use crate::{
    actors::Actor,
    objects::{Object, ObjectAttachmentSlotData, PlayerObject},
    types::colour::Colour,
    types::vector::{Vector3, Vector4},
    vehicles::Vehicle,
};

native!(SendClientMessage, player: struct Player, colour: Colour, message: str);
native!(GetPlayerName, player: struct Player, name: mut str, -> isize);
native!(SendClientMessageToAll, colour: Colour, message: str);
native!(SetPlayerCameraPos, player: struct Player, pos: Vector3);
native!(SetPlayerDrunkLevel, player: struct Player, level: isize);
native!(SetPlayerInterior, player: struct Player, interiorid: isize);
native!(SetPlayerWantedLevel, player: struct Player, level: isize);
native!(SetPlayerWeather, player: struct Player, weatherid: isize);
native!(GetPlayerWeather, player: struct Player, -> isize);
native!(SetPlayerSkin, player: struct Player, skinid: isize);
native!(SetPlayerShopName, player: struct Player, shopname: str);
native!(GivePlayerMoney, player: struct Player, amount: isize);
native!(SetPlayerCameraLookAt, player: struct Player,pos:Vector3, cut: PlayerCameraCutType);
native!(SetCameraBehindPlayer, player: struct Player);
native!(CreateExplosionForPlayer, player: struct Player, pos: Vector3, explosion_type: isize, radius: f32);
native!(PlayAudioStreamForPlayer, player: struct Player, url: str,pos: Vector3, distance: f32, usePos: bool);
native!(StopAudioStreamForPlayer, player: struct Player);
native!(SendDeathMessage, killer:struct Player, killee:struct Player, weapon: PlayerWeapon);
native!(TogglePlayerWidescreen, player: struct Player, enable: bool);
native!(IsPlayerWidescreenToggled, player: struct Player, -> bool);
native!(SetPlayerHealth, player: struct Player, health: f32);
native!(GetPlayerHealth, player: struct Player, -> f32);
native!(SetPlayerArmour, player: struct Player, armour: f32);
native!(GetPlayerArmour, player: struct Player, -> f32);
native!(SetPlayerTeam, player: struct Player, teamid: isize);
native!(GetPlayerTeam, player: struct Player, -> isize);
native!(SetPlayerScore, player: struct Player, score: isize);
native!(GetPlayerScore, player: struct Player, -> isize);
native!(GetPlayerSkin, player: struct Player, -> isize);
native!(SetPlayerColor, player: struct Player, colour: Colour);
native!(GetPlayerColor, player: struct Player, -> isize);
native!(GetDefaultPlayerColour, player: struct Player, -> isize);
native!(GetPlayerDrunkLevel, player: struct Player, -> isize);
native!(GivePlayerWeapon, player: struct Player, data: WeaponSlotData);
native!(RemovePlayerWeapon, player: struct Player, weaponid:u8);
native!(GetPlayerMoney, player: struct Player, -> isize);
native!(ResetPlayerMoney, player: struct Player);
native!(SetPlayerName, player: struct Player, name: str, -> PlayerNameStatus);
native!(GetPlayerState, player: struct Player, -> PlayerState);
native!(GetPlayerPing, player: struct Player, -> isize);
native!(GetPlayerWeapon, player: struct Player, -> PlayerWeapon);
native!(SetPlayerTime, player: struct Player, hour: isize, minute: isize);
native!(GetPlayerTime, player: struct Player, hour: mut isize, minute: mut isize);
native!(TogglePlayerClock, player: struct Player, enable: bool);
native!(PlayerHasClockEnabled, player: struct Player, -> bool);
native!(ForceClassSelection, player: struct Player);
native!(GetPlayerWantedLevel, player: struct Player, -> isize);
native!(SetPlayerFightingStyle, player: struct Player, style: PlayerFightingStyle);
native!(GetPlayerFightingStyle, player: struct Player, -> PlayerFightingStyle);
native!(SetPlayerVelocity, player: struct Player,pos: Vector3);
native!(GetPlayerVelocity, player: struct Player,pos: mut Vector3);
native!(GetPlayerCameraPos, player: struct Player,pos: mut Vector3);
native!(GetPlayerDistanceFromPoint, player: struct Player, pos: Vector3, -> f32);
native!(GetPlayerInterior, player: struct Player, -> isize);
native!(SetPlayerPos, player: struct Player, pos: Vector3);
native!(GetPlayerPos, player: struct Player,pos: mut Vector3);
native!(GetPlayerVirtualWorld, player: struct Player, -> isize);
native!(IsPlayerNPC, player: struct Player, -> bool);
native!(IsPlayerStreamedIn, player: struct Player, other:struct Player, -> bool);
native!(PlayerPlaySound, player: struct Player, sound:usize, pos: Vector3);
native!(PlayerSpectatePlayer, player: struct Player, target:struct Player, mode: PlayerSpectateMode);
native!(PlayerSpectateVehicle, player: struct Player, vehicle:struct Vehicle, mode: PlayerSpectateMode);
native!(SetPlayerVirtualWorld, player: struct Player, vw: isize);
native!(SetPlayerWorldBounds, player: struct Player, coords: Vector4);
native!(ClearPlayerWorldBounds, player: struct Player);
native!(GetPlayerWorldBounds, player: struct Player,pos: mut Vector4);
native!(ClearAnimations, player: struct Player, syncType: PlayerAnimationSyncType);
native!(GetPlayerLastShotVectors, player: struct Player, -> PlayerBulletData);
native!(GetPlayerCameraTargetPlayer, player: struct Player, -> struct Player);
native!(GetPlayerCameraTargetActor, player: struct Player, -> struct Actor);
native!(GetPlayerCameraTargetObject, player: struct Player, -> struct Object);
native!(GetPlayerCameraTargetVehicle, player: struct Player, -> struct Vehicle);
native!(PutPlayerInVehicle, player: struct Player, vehicle:struct Vehicle, seatID: isize);
native!(RemoveBuildingForPlayer, player: struct Player, model: isize,pos: Vector3, radius: f32);
native!(GetPlayerBuildingsRemoved, player: struct Player, -> isize);
native!(RemovePlayerFromVehicle, player: struct Player, force: bool);
native!(RemovePlayerMapIcon, player: struct Player, iconID: isize);
native!(SetPlayerMapIcon, player: struct Player, iconID: isize, pos: Vector3, icon_type: isize, colour: Colour, style: MapIconStyle);
native!(ResetPlayerWeapons, player: struct Player);
native!(SetPlayerAmmo, player: struct Player, data: WeaponSlotData);
native!(SetPlayerArmedWeapon, player: struct Player, weapon: PlayerWeapon);
native!(SetPlayerChatBubble, player: struct Player, text: str, colour: Colour, drawdistance: f32, expiretime: isize);
native!(SetPlayerPosFindZ, player: struct Player, pos: Vector3);
native!(SetPlayerSkillLevel, player: struct Player, weapon: PlayerWeaponSkill, level: isize);
native!(SetPlayerSpecialAction, player: struct Player, action: PlayerSpecialAction);
native!(ShowPlayerNameTagForPlayer, player: struct Player, other:struct Player, enable: bool);
native!(TogglePlayerControllable, player: struct Player, enable: bool);
native!(TogglePlayerSpectating, player: struct Player, enable: bool);
native!(ApplyAnimation,player:struct Player, animlib: str, animname:str, delta:f32, animloop:bool, lockX:bool, lockY:bool ,freeze:bool, time:usize, sync:PlayerAnimationSyncType);
native!(EditAttachedObject, player: struct Player, index: isize);
native!(EnablePlayerCameraTarget, player: struct Player, enable: bool);
native!(EnableStuntBonusForPlayer, player: struct Player, enable: bool);
native!(GetPlayerAmmo, player: struct Player, -> isize);
native!(GetPlayerAnimationIndex, player: struct Player, -> isize);
native!(GetPlayerFacingAngle, player: struct Player, -> f32);
native!(GetPlayerIp, player: struct Player, ip: mut str, -> isize);
native!(GetPlayerSpecialAction, player: struct Player, -> PlayerSpecialAction);
native!(GetPlayerVehicleID, player: struct Player, -> isize);
native!(GetPlayerVehicleSeat, player: struct Player, -> isize);
native!(GetPlayerWeaponData, player: struct Player, slot: isize, weapon: mut WeaponSlotData);
native!(GetPlayerWeaponState, player: struct Player, -> isize);
native!(InterpolateCameraPos, player: struct Player, from: Vector3, to: Vector3, time: isize, cut: PlayerCameraCutType);
native!(InterpolateCameraLookAt, player: struct Player, from: Vector3, to: Vector3, time: isize, cut: PlayerCameraCutType, -> bool);
native!(IsPlayerAttachedObjectSlotUsed, player: struct Player, index: isize, -> bool);
native!(AttachCameraToObject, player: struct Player, object:struct Object);
native!(AttachCameraToPlayerObject, player: struct Player, object:struct PlayerObject);
native!(GetPlayerAimData, player: struct Player, -> PlayerAimData);
native!(GetPlayerKeys, player: struct Player, keys: mut isize, updown: mut isize, leftright: mut isize, -> PlayerKeyData);
native!(GetPlayerSurfingData, player: struct Player, -> PlayerSurfingData);
native!(GetPlayerTargetPlayer, player: struct Player, -> struct Player);
native!(GetPlayerTargetActor, player: struct Player, -> struct Actor);
native!(IsPlayerInVehicle, player: struct Player, targetVehicle:struct Vehicle, -> bool);
native!(IsPlayerInAnyVehicle, player: struct Player, -> bool);
native!(IsPlayerInRangeOfPoint, player: struct Player, range: f32, coord: Vector3, -> bool);
native!(PlayCrimeReportForPlayer, player: struct Player, suspect:struct Player, crime: isize, -> bool);
native!(RemovePlayerAttachedObject, player: struct Player, index: isize);
native!(SetPlayerAttachedObject, player: struct Player, index: isize, attachment: ObjectAttachmentSlotData);
native!(GetPlayerAttachedObject, player: struct Player, index: isize, -> ObjectAttachmentSlotData);
native!(SetPlayerFacingAngle, player: struct Player, angle: f32);
native!(SetPlayerMarkerForPlayer, player: struct Player, other:struct Player, colour: Colour);
native!(GetPlayerMarkerForPlayer, player: struct Player, other:struct Player, -> isize);
native!(AllowPlayerTeleport, player: struct Player, allow: bool);
native!(IsPlayerTeleportAllowed, player: struct Player, -> bool);
native!(SetRemoteVehicleCollisions, player: struct Player, collide: bool);
native!(SelectTextDraw, player: struct Player, hoverColour: Colour);
native!(CancelSelectTextDraw, player: struct Player);
native!(SendClientCheck, player: struct Player, actionType: isize, address: isize, offset: isize, count: isize);
native!(SpawnPlayer, player: struct Player);
native!(gpci, player: struct Player, output: mut str);
native!(IsPlayerAdmin, player: struct Player, -> bool);
native!(Kick, player: struct Player);
native!(GameTextForPlayer, player: struct Player, msg: str, time: isize, style: isize);
native!(HideGameTextForPlayer, player: struct Player, style: isize);
native!(HasGameText, player: struct Player, style: isize, -> bool);
native!(GetGameText, player: struct Player, style: isize, message: mut str, time: mut isize, remaining: mut isize, -> bool);
native!(Ban, player: struct Player);
native!(BanEx, player: struct Player, msg: str);
native!(SendDeathMessageToPlayer, player: struct Player, killer:struct Player, killee:struct Player, weapon: PlayerWeapon);
native!(SendPlayerMessageToPlayer, player: struct Player, sender:struct Player, message: str);
native!(GetPlayerVersion, player: struct Player, output: mut str);
native!(GetPlayerSkillLevel, player: struct Player, skill: isize, -> isize);
native!(GetPlayerSpectateID, player: struct Player, -> isize);
native!(GetPlayerSpectateData, player: struct Player, -> PlayerSpectateData);
native!(GetPlayerRawIp, player: struct Player, -> isize);
native!(SetPlayerGravity, player: struct Player, gravity: f32);
native!(GetPlayerGravity, player: struct Player, -> f32);
native!(SetPlayerAdmin, player: struct Player, set: bool);
native!(IsPlayerSpawned, player: struct Player, -> bool);
native!(IsPlayerControllable, player: struct Player, -> bool);
native!(IsPlayerCameraTargetEnabled, player: struct Player, -> bool);
native!(TogglePlayerGhostMode, player: struct Player, toggle: bool);
native!(GetPlayerGhostMode, player: struct Player, -> bool);
native!(AllowPlayerWeapons, player: struct Player, allow: bool);
native!(ArePlayerWeaponsAllowed, player: struct Player, -> bool);
native!(IsPlayerUsingOfficialClient, player: struct Player, -> bool);
native!(GetPlayerAnimationData, player: struct Player, -> PlayerAnimationData);
native!(IsPlayerInDriveByMode, player: struct Player, -> bool);
native!(IsPlayerCuffed, player: struct Player, -> bool);
native!(GetPlayerCustomSkin,player: struct Player,-> isize);
native!(RedirectDownload,player: struct Player, url: str, -> bool);
native!(GetPlayerID,player:struct Player, -> usize);
native!(NetStats_BytesReceived, player: struct Player, -> isize);
native!(NetStats_BytesSent, player: struct Player, -> isize);
native!(NetStats_ConnectionStatus, player: struct Player, -> isize);
native!(NetStats_GetConnectedTime, player: struct Player, -> isize);
native!(NetStats_GetIpPort, player: struct Player, output: mut str);
native!(NetStats_MessagesReceived, player: struct Player, -> isize);
native!(NetStats_MessagesRecvPerSecond, player: struct Player, -> isize);
native!(NetStats_MessagesSent, player: struct Player, -> isize);
native!(NetStats_PacketLossPercent, player: struct Player, -> f32);
native!(SendPlayerMessageToAll, player: struct Player, message: str);
native!(GetPlayerFromID,playerid:isize, -> struct Player);

#[doc(hidden)]
pub fn load_functions() {
    load_function!(SendClientMessage);
    load_function!(GetPlayerName);
    load_function!(SendClientMessageToAll);
    load_function!(SetPlayerCameraPos);
    load_function!(SetPlayerDrunkLevel);
    load_function!(SetPlayerInterior);
    load_function!(SetPlayerWantedLevel);
    load_function!(SetPlayerWeather);
    load_function!(GetPlayerWeather);
    load_function!(SetPlayerSkin);
    load_function!(SetPlayerShopName);
    load_function!(GivePlayerMoney);
    load_function!(SetPlayerCameraLookAt);
    load_function!(SetCameraBehindPlayer);
    load_function!(CreateExplosionForPlayer);
    load_function!(PlayAudioStreamForPlayer);
    load_function!(StopAudioStreamForPlayer);
    load_function!(SendDeathMessage);
    load_function!(TogglePlayerWidescreen);
    load_function!(IsPlayerWidescreenToggled);
    load_function!(SetPlayerHealth);
    load_function!(GetPlayerHealth);
    load_function!(SetPlayerArmour);
    load_function!(GetPlayerArmour);
    load_function!(SetPlayerTeam);
    load_function!(GetPlayerTeam);
    load_function!(SetPlayerScore);
    load_function!(GetPlayerScore);
    load_function!(GetPlayerSkin);
    load_function!(SetPlayerColor);
    load_function!(GetPlayerColor);
    load_function!(GetDefaultPlayerColour);
    load_function!(GetPlayerDrunkLevel);
    load_function!(GivePlayerWeapon);
    load_function!(RemovePlayerWeapon);
    load_function!(GetPlayerMoney);
    load_function!(ResetPlayerMoney);
    load_function!(SetPlayerName);
    load_function!(GetPlayerState);
    load_function!(GetPlayerPing);
    load_function!(GetPlayerWeapon);
    load_function!(SetPlayerTime);
    load_function!(GetPlayerTime);
    load_function!(TogglePlayerClock);
    load_function!(PlayerHasClockEnabled);
    load_function!(ForceClassSelection);
    load_function!(GetPlayerWantedLevel);
    load_function!(SetPlayerFightingStyle);
    load_function!(GetPlayerFightingStyle);
    load_function!(SetPlayerVelocity);
    load_function!(GetPlayerVelocity);
    load_function!(GetPlayerCameraPos);
    load_function!(GetPlayerDistanceFromPoint);
    load_function!(GetPlayerInterior);
    load_function!(SetPlayerPos);
    load_function!(GetPlayerPos);
    load_function!(GetPlayerVirtualWorld);
    load_function!(IsPlayerNPC);
    load_function!(IsPlayerStreamedIn);
    load_function!(PlayerPlaySound);
    load_function!(PlayerSpectatePlayer);
    load_function!(PlayerSpectateVehicle);
    load_function!(SetPlayerVirtualWorld);
    load_function!(SetPlayerWorldBounds);
    load_function!(ClearPlayerWorldBounds);
    load_function!(GetPlayerWorldBounds);
    load_function!(ClearAnimations);
    load_function!(GetPlayerLastShotVectors);
    load_function!(GetPlayerCameraTargetPlayer);
    load_function!(GetPlayerCameraTargetActor);
    load_function!(GetPlayerCameraTargetObject);
    load_function!(GetPlayerCameraTargetVehicle);
    load_function!(PutPlayerInVehicle);
    load_function!(RemoveBuildingForPlayer);
    load_function!(GetPlayerBuildingsRemoved);
    load_function!(RemovePlayerFromVehicle);
    load_function!(RemovePlayerMapIcon);
    load_function!(SetPlayerMapIcon);
    load_function!(ResetPlayerWeapons);
    load_function!(SetPlayerAmmo);
    load_function!(SetPlayerArmedWeapon);
    load_function!(SetPlayerChatBubble);
    load_function!(SetPlayerPosFindZ);
    load_function!(SetPlayerSkillLevel);
    load_function!(SetPlayerSpecialAction);
    load_function!(ShowPlayerNameTagForPlayer);
    load_function!(TogglePlayerControllable);
    load_function!(TogglePlayerSpectating);
    load_function!(ApplyAnimation);
    load_function!(EditAttachedObject);
    load_function!(EnablePlayerCameraTarget);
    load_function!(EnableStuntBonusForPlayer);
    load_function!(GetPlayerAmmo);
    load_function!(GetPlayerAnimationIndex);
    load_function!(GetPlayerFacingAngle);
    load_function!(GetPlayerIp);
    load_function!(GetPlayerSpecialAction);
    load_function!(GetPlayerVehicleID);
    load_function!(GetPlayerVehicleSeat);
    load_function!(GetPlayerWeaponData);
    load_function!(GetPlayerWeaponState);
    load_function!(InterpolateCameraPos);
    load_function!(InterpolateCameraLookAt);
    load_function!(IsPlayerAttachedObjectSlotUsed);
    load_function!(AttachCameraToObject);
    load_function!(AttachCameraToPlayerObject);
    load_function!(GetPlayerAimData);
    load_function!(GetPlayerKeys);
    load_function!(GetPlayerSurfingData);
    load_function!(GetPlayerTargetPlayer);
    load_function!(GetPlayerTargetActor);
    load_function!(IsPlayerInVehicle);
    load_function!(IsPlayerInAnyVehicle);
    load_function!(IsPlayerInRangeOfPoint);
    load_function!(PlayCrimeReportForPlayer);
    load_function!(RemovePlayerAttachedObject);
    load_function!(SetPlayerAttachedObject);
    load_function!(GetPlayerAttachedObject);
    load_function!(SetPlayerFacingAngle);
    load_function!(SetPlayerMarkerForPlayer);
    load_function!(GetPlayerMarkerForPlayer);
    load_function!(AllowPlayerTeleport);
    load_function!(IsPlayerTeleportAllowed);
    load_function!(SetRemoteVehicleCollisions);
    load_function!(SelectTextDraw);
    load_function!(CancelSelectTextDraw);
    load_function!(SendClientCheck);
    load_function!(SpawnPlayer);
    load_function!(gpci);
    load_function!(IsPlayerAdmin);
    load_function!(Kick);
    load_function!(GameTextForPlayer);
    load_function!(HideGameTextForPlayer);
    load_function!(HasGameText);
    load_function!(GetGameText);
    load_function!(Ban);
    load_function!(BanEx);
    load_function!(SendDeathMessageToPlayer);
    load_function!(SendPlayerMessageToPlayer);
    load_function!(GetPlayerVersion);
    load_function!(GetPlayerSkillLevel);
    load_function!(GetPlayerSpectateID);
    load_function!(GetPlayerSpectateData);
    load_function!(GetPlayerRawIp);
    load_function!(SetPlayerGravity);
    load_function!(GetPlayerGravity);
    load_function!(SetPlayerAdmin);
    load_function!(IsPlayerSpawned);
    load_function!(IsPlayerControllable);
    load_function!(IsPlayerCameraTargetEnabled);
    load_function!(TogglePlayerGhostMode);
    load_function!(GetPlayerGhostMode);
    load_function!(AllowPlayerWeapons);
    load_function!(ArePlayerWeaponsAllowed);
    load_function!(IsPlayerUsingOfficialClient);
    load_function!(GetPlayerAnimationData);
    load_function!(IsPlayerInDriveByMode);
    load_function!(IsPlayerCuffed);
    load_function!(GetPlayerCustomSkin);
    load_function!(RedirectDownload);
    //load_function!(IsValidPlayer);
    load_function!(GetPlayerID);
    load_function!(NetStats_BytesReceived);
    load_function!(NetStats_BytesSent);
    load_function!(NetStats_ConnectionStatus);
    load_function!(NetStats_GetConnectedTime);
    load_function!(NetStats_GetIpPort);
    load_function!(NetStats_MessagesReceived);
    load_function!(NetStats_MessagesRecvPerSecond);
    load_function!(NetStats_MessagesSent);
    load_function!(NetStats_PacketLossPercent);
    load_function!(SendPlayerMessageToAll);
    load_function!(GetPlayerFromID);
}
