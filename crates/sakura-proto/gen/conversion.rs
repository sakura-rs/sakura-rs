#[allow(unused, warnings)]
pub fn client_to_normal(
    cmd_id: u16,
    body: &[u8],
) -> Result<(u16, Box<[u8]>), ProtocolConversionError> {
    use crate::packet::{read_cmd_id, NetPacket};
    use crate::{Protobuf, CmdID};
    match cmd_id {
        crate::client::WearEquipRsp::CMD_ID => {
            let proto = crate::client::WearEquipRsp::decode(body)?;
            let proto: crate::normal::WearEquipRsp = proto.into();
            Ok((
                crate::normal::WearEquipRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::MarkMapRsp::CMD_ID => {
            let proto = crate::client::MarkMapRsp::decode(body)?;
            let proto: crate::normal::MarkMapRsp = proto.into();
            Ok((
                crate::normal::MarkMapRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::EnterSceneDoneRsp::CMD_ID => {
            let proto = crate::client::EnterSceneDoneRsp::decode(body)?;
            let proto: crate::normal::EnterSceneDoneRsp = proto.into();
            Ok((
                crate::normal::EnterSceneDoneRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::SyncScenePlayTeamEntityNotify::CMD_ID => {
            let proto = crate::client::SyncScenePlayTeamEntityNotify::decode(body)?;
            let proto: crate::normal::SyncScenePlayTeamEntityNotify = proto.into();
            Ok((
                crate::normal::SyncScenePlayTeamEntityNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::WearEquipReq::CMD_ID => {
            let proto = crate::client::WearEquipReq::decode(body)?;
            let proto: crate::normal::WearEquipReq = proto.into();
            Ok((
                crate::normal::WearEquipReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::EvtAvatarStandUpNotify::CMD_ID => {
            let proto = crate::client::EvtAvatarStandUpNotify::decode(body)?;
            let proto: crate::normal::EvtAvatarStandUpNotify = proto.into();
            Ok((
                crate::normal::EvtAvatarStandUpNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::EnterSceneReadyReq::CMD_ID => {
            let proto = crate::client::EnterSceneReadyReq::decode(body)?;
            let proto: crate::normal::EnterSceneReadyReq = proto.into();
            Ok((
                crate::normal::EnterSceneReadyReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::ClientSetGameTimeRsp::CMD_ID => {
            let proto = crate::client::ClientSetGameTimeRsp::decode(body)?;
            let proto: crate::normal::ClientSetGameTimeRsp = proto.into();
            Ok((
                crate::normal::ClientSetGameTimeRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::PlayerGameTimeNotify::CMD_ID => {
            let proto = crate::client::PlayerGameTimeNotify::decode(body)?;
            let proto: crate::normal::PlayerGameTimeNotify = proto.into();
            Ok((
                crate::normal::PlayerGameTimeNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::AvatarEquipChangeNotify::CMD_ID => {
            let proto = crate::client::AvatarEquipChangeNotify::decode(body)?;
            let proto: crate::normal::AvatarEquipChangeNotify = proto.into();
            Ok((
                crate::normal::AvatarEquipChangeNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::GetScenePointRsp::CMD_ID => {
            let proto = crate::client::GetScenePointRsp::decode(body)?;
            let proto: crate::normal::GetScenePointRsp = proto.into();
            Ok((
                crate::normal::GetScenePointRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::ClientLoadingCostumeVerificationNotify::CMD_ID => {
            let proto = crate::client::ClientLoadingCostumeVerificationNotify::decode(
                body,
            )?;
            let proto: crate::normal::ClientLoadingCostumeVerificationNotify = proto
                .into();
            Ok((
                crate::normal::ClientLoadingCostumeVerificationNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::AvatarWearFlycloakRsp::CMD_ID => {
            let proto = crate::client::AvatarWearFlycloakRsp::decode(body)?;
            let proto: crate::normal::AvatarWearFlycloakRsp = proto.into();
            Ok((
                crate::normal::AvatarWearFlycloakRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::ClientSetGameTimeReq::CMD_ID => {
            let proto = crate::client::ClientSetGameTimeReq::decode(body)?;
            let proto: crate::normal::ClientSetGameTimeReq = proto.into();
            Ok((
                crate::normal::ClientSetGameTimeReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::AvatarDataNotify::CMD_ID => {
            let proto = crate::client::AvatarDataNotify::decode(body)?;
            let proto: crate::normal::AvatarDataNotify = proto.into();
            Ok((
                crate::normal::AvatarDataNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::EvtAvatarLockChairReq::CMD_ID => {
            let proto = crate::client::EvtAvatarLockChairReq::decode(body)?;
            let proto: crate::normal::EvtAvatarLockChairReq = proto.into();
            Ok((
                crate::normal::EvtAvatarLockChairReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::SceneEntityAppearNotify::CMD_ID => {
            let proto = crate::client::SceneEntityAppearNotify::decode(body)?;
            let proto: crate::normal::SceneEntityAppearNotify = proto.into();
            Ok((
                crate::normal::SceneEntityAppearNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::PingReq::CMD_ID => {
            let proto = crate::client::PingReq::decode(body)?;
            let proto: crate::normal::PingReq = proto.into();
            Ok((
                crate::normal::PingReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::UnionCmdNotify::CMD_ID => {
            let proto = crate::client::UnionCmdNotify::decode(body)?;
            let proto: crate::normal::UnionCmdNotify = proto.into();
            Ok((
                crate::normal::UnionCmdNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::AvatarFlycloakChangeNotify::CMD_ID => {
            let proto = crate::client::AvatarFlycloakChangeNotify::decode(body)?;
            let proto: crate::normal::AvatarFlycloakChangeNotify = proto.into();
            Ok((
                crate::normal::AvatarFlycloakChangeNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::SceneTimeNotify::CMD_ID => {
            let proto = crate::client::SceneTimeNotify::decode(body)?;
            let proto: crate::normal::SceneTimeNotify = proto.into();
            Ok((
                crate::normal::SceneTimeNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::OpenStateUpdateNotify::CMD_ID => {
            let proto = crate::client::OpenStateUpdateNotify::decode(body)?;
            let proto: crate::normal::OpenStateUpdateNotify = proto.into();
            Ok((
                crate::normal::OpenStateUpdateNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::AvatarChangeTraceEffectReq::CMD_ID => {
            let proto = crate::client::AvatarChangeTraceEffectReq::decode(body)?;
            let proto: crate::normal::AvatarChangeTraceEffectReq = proto.into();
            Ok((
                crate::normal::AvatarChangeTraceEffectReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::QueryPathReq::CMD_ID => {
            let proto = crate::client::QueryPathReq::decode(body)?;
            let proto: crate::normal::QueryPathReq = proto.into();
            Ok((
                crate::normal::QueryPathReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::EvtBeingHitNotify::CMD_ID => {
            let proto = crate::client::EvtBeingHitNotify::decode(body)?;
            let proto: crate::normal::EvtBeingHitNotify = proto.into();
            Ok((
                crate::normal::EvtBeingHitNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::NpcTalkRsp::CMD_ID => {
            let proto = crate::client::NpcTalkRsp::decode(body)?;
            let proto: crate::normal::NpcTalkRsp = proto.into();
            Ok((
                crate::normal::NpcTalkRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::LifeStateChangeNotify::CMD_ID => {
            let proto = crate::client::LifeStateChangeNotify::decode(body)?;
            let proto: crate::normal::LifeStateChangeNotify = proto.into();
            Ok((
                crate::normal::LifeStateChangeNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::PathfindingEnterSceneReq::CMD_ID => {
            let proto = crate::client::PathfindingEnterSceneReq::decode(body)?;
            let proto: crate::normal::PathfindingEnterSceneReq = proto.into();
            Ok((
                crate::normal::PathfindingEnterSceneReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::GetScenePointReq::CMD_ID => {
            let proto = crate::client::GetScenePointReq::decode(body)?;
            let proto: crate::normal::GetScenePointReq = proto.into();
            Ok((
                crate::normal::GetScenePointReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::GetPlayerTokenRsp::CMD_ID => {
            let proto = crate::client::GetPlayerTokenRsp::decode(body)?;
            let proto: crate::normal::GetPlayerTokenRsp = proto.into();
            Ok((
                crate::normal::GetPlayerTokenRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::ChangeAvatarReq::CMD_ID => {
            let proto = crate::client::ChangeAvatarReq::decode(body)?;
            let proto: crate::normal::ChangeAvatarReq = proto.into();
            Ok((
                crate::normal::ChangeAvatarReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::NpcTalkReq::CMD_ID => {
            let proto = crate::client::NpcTalkReq::decode(body)?;
            let proto: crate::normal::NpcTalkReq = proto.into();
            Ok((
                crate::normal::NpcTalkReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::SceneInitFinishRsp::CMD_ID => {
            let proto = crate::client::SceneInitFinishRsp::decode(body)?;
            let proto: crate::normal::SceneInitFinishRsp = proto.into();
            Ok((
                crate::normal::SceneInitFinishRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::PlayerDataNotify::CMD_ID => {
            let proto = crate::client::PlayerDataNotify::decode(body)?;
            let proto: crate::normal::PlayerDataNotify = proto.into();
            Ok((
                crate::normal::PlayerDataNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::MarkMapReq::CMD_ID => {
            let proto = crate::client::MarkMapReq::decode(body)?;
            let proto: crate::normal::MarkMapReq = proto.into();
            Ok((
                crate::normal::MarkMapReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::PlayerLoginReq::CMD_ID => {
            let proto = crate::client::PlayerLoginReq::decode(body)?;
            let proto: crate::normal::PlayerLoginReq = proto.into();
            Ok((
                crate::normal::PlayerLoginReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::AvatarChangeTraceEffectRsp::CMD_ID => {
            let proto = crate::client::AvatarChangeTraceEffectRsp::decode(body)?;
            let proto: crate::normal::AvatarChangeTraceEffectRsp = proto.into();
            Ok((
                crate::normal::AvatarChangeTraceEffectRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::EnterScenePeerNotify::CMD_ID => {
            let proto = crate::client::EnterScenePeerNotify::decode(body)?;
            let proto: crate::normal::EnterScenePeerNotify = proto.into();
            Ok((
                crate::normal::EnterScenePeerNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::WindSeedClientNotify::CMD_ID => {
            let proto = crate::client::WindSeedClientNotify::decode(body)?;
            let proto: crate::normal::WindSeedClientNotify = proto.into();
            Ok((
                crate::normal::WindSeedClientNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::GetSceneAreaRsp::CMD_ID => {
            let proto = crate::client::GetSceneAreaRsp::decode(body)?;
            let proto: crate::normal::GetSceneAreaRsp = proto.into();
            Ok((
                crate::normal::GetSceneAreaRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::AvatarChangeCostumeNotify::CMD_ID => {
            let proto = crate::client::AvatarChangeCostumeNotify::decode(body)?;
            let proto: crate::normal::AvatarChangeCostumeNotify = proto.into();
            Ok((
                crate::normal::AvatarChangeCostumeNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::EvtAvatarLockChairRsp::CMD_ID => {
            let proto = crate::client::EvtAvatarLockChairRsp::decode(body)?;
            let proto: crate::normal::EvtAvatarLockChairRsp = proto.into();
            Ok((
                crate::normal::EvtAvatarLockChairRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::EnterSceneReadyRsp::CMD_ID => {
            let proto = crate::client::EnterSceneReadyRsp::decode(body)?;
            let proto: crate::normal::EnterSceneReadyRsp = proto.into();
            Ok((
                crate::normal::EnterSceneReadyRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::PlayerNormalLuaShellNotify::CMD_ID => {
            let proto = crate::client::PlayerNormalLuaShellNotify::decode(body)?;
            let proto: crate::normal::PlayerNormalLuaShellNotify = proto.into();
            Ok((
                crate::normal::PlayerNormalLuaShellNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::PlayerStoreNotify::CMD_ID => {
            let proto = crate::client::PlayerStoreNotify::decode(body)?;
            let proto: crate::normal::PlayerStoreNotify = proto.into();
            Ok((
                crate::normal::PlayerStoreNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::SceneInitFinishReq::CMD_ID => {
            let proto = crate::client::SceneInitFinishReq::decode(body)?;
            let proto: crate::normal::SceneInitFinishReq = proto.into();
            Ok((
                crate::normal::SceneInitFinishReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::EntityFightPropUpdateNotify::CMD_ID => {
            let proto = crate::client::EntityFightPropUpdateNotify::decode(body)?;
            let proto: crate::normal::EntityFightPropUpdateNotify = proto.into();
            Ok((
                crate::normal::EntityFightPropUpdateNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::SetUpAvatarTeamReq::CMD_ID => {
            let proto = crate::client::SetUpAvatarTeamReq::decode(body)?;
            let proto: crate::normal::SetUpAvatarTeamReq = proto.into();
            Ok((
                crate::normal::SetUpAvatarTeamReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::SceneForceUnlockNotify::CMD_ID => {
            let proto = crate::client::SceneForceUnlockNotify::decode(body)?;
            let proto: crate::normal::SceneForceUnlockNotify = proto.into();
            Ok((
                crate::normal::SceneForceUnlockNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::HomeAvatarCostumeChangeNotify::CMD_ID => {
            let proto = crate::client::HomeAvatarCostumeChangeNotify::decode(body)?;
            let proto: crate::normal::HomeAvatarCostumeChangeNotify = proto.into();
            Ok((
                crate::normal::HomeAvatarCostumeChangeNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::ScenePointUnlockNotify::CMD_ID => {
            let proto = crate::client::ScenePointUnlockNotify::decode(body)?;
            let proto: crate::normal::ScenePointUnlockNotify = proto.into();
            Ok((
                crate::normal::ScenePointUnlockNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::GetPlayerTokenReq::CMD_ID => {
            let proto = crate::client::GetPlayerTokenReq::decode(body)?;
            let proto: crate::normal::GetPlayerTokenReq = proto.into();
            Ok((
                crate::normal::GetPlayerTokenReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::UnlockTransPointReq::CMD_ID => {
            let proto = crate::client::UnlockTransPointReq::decode(body)?;
            let proto: crate::normal::UnlockTransPointReq = proto.into();
            Ok((
                crate::normal::UnlockTransPointReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::AvatarChangeTraceEffectNotify::CMD_ID => {
            let proto = crate::client::AvatarChangeTraceEffectNotify::decode(body)?;
            let proto: crate::normal::AvatarChangeTraceEffectNotify = proto.into();
            Ok((
                crate::normal::AvatarChangeTraceEffectNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::AvatarFightPropUpdateNotify::CMD_ID => {
            let proto = crate::client::AvatarFightPropUpdateNotify::decode(body)?;
            let proto: crate::normal::AvatarFightPropUpdateNotify = proto.into();
            Ok((
                crate::normal::AvatarFightPropUpdateNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::SceneTeamUpdateNotify::CMD_ID => {
            let proto = crate::client::SceneTeamUpdateNotify::decode(body)?;
            let proto: crate::normal::SceneTeamUpdateNotify = proto.into();
            Ok((
                crate::normal::SceneTeamUpdateNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::CombatInvocationsNotify::CMD_ID => {
            let proto = crate::client::CombatInvocationsNotify::decode(body)?;
            let proto: crate::normal::CombatInvocationsNotify = proto.into();
            Ok((
                crate::normal::CombatInvocationsNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::QueryPathRsp::CMD_ID => {
            let proto = crate::client::QueryPathRsp::decode(body)?;
            let proto: crate::normal::QueryPathRsp = proto.into();
            Ok((
                crate::normal::QueryPathRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::AvatarTeamUpdateNotify::CMD_ID => {
            let proto = crate::client::AvatarTeamUpdateNotify::decode(body)?;
            let proto: crate::normal::AvatarTeamUpdateNotify = proto.into();
            Ok((
                crate::normal::AvatarTeamUpdateNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::PlayerLoginRsp::CMD_ID => {
            let proto = crate::client::PlayerLoginRsp::decode(body)?;
            let proto: crate::normal::PlayerLoginRsp = proto.into();
            Ok((
                crate::normal::PlayerLoginRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::GetAreaExplorePointRsp::CMD_ID => {
            let proto = crate::client::GetAreaExplorePointRsp::decode(body)?;
            let proto: crate::normal::GetAreaExplorePointRsp = proto.into();
            Ok((
                crate::normal::GetAreaExplorePointRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::GetSceneAreaReq::CMD_ID => {
            let proto = crate::client::GetSceneAreaReq::decode(body)?;
            let proto: crate::normal::GetSceneAreaReq = proto.into();
            Ok((
                crate::normal::GetSceneAreaReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::PingRsp::CMD_ID => {
            let proto = crate::client::PingRsp::decode(body)?;
            let proto: crate::normal::PingRsp = proto.into();
            Ok((
                crate::normal::PingRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::SceneEntityDisappearNotify::CMD_ID => {
            let proto = crate::client::SceneEntityDisappearNotify::decode(body)?;
            let proto: crate::normal::SceneEntityDisappearNotify = proto.into();
            Ok((
                crate::normal::SceneEntityDisappearNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::PostEnterSceneRsp::CMD_ID => {
            let proto = crate::client::PostEnterSceneRsp::decode(body)?;
            let proto: crate::normal::PostEnterSceneRsp = proto.into();
            Ok((
                crate::normal::PostEnterSceneRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::PostEnterSceneReq::CMD_ID => {
            let proto = crate::client::PostEnterSceneReq::decode(body)?;
            let proto: crate::normal::PostEnterSceneReq = proto.into();
            Ok((
                crate::normal::PostEnterSceneReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::PlayerEnterSceneInfoNotify::CMD_ID => {
            let proto = crate::client::PlayerEnterSceneInfoNotify::decode(body)?;
            let proto: crate::normal::PlayerEnterSceneInfoNotify = proto.into();
            Ok((
                crate::normal::PlayerEnterSceneInfoNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::EntityFightPropNotify::CMD_ID => {
            let proto = crate::client::EntityFightPropNotify::decode(body)?;
            let proto: crate::normal::EntityFightPropNotify = proto.into();
            Ok((
                crate::normal::EntityFightPropNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::AvatarChangeCostumeReq::CMD_ID => {
            let proto = crate::client::AvatarChangeCostumeReq::decode(body)?;
            let proto: crate::normal::AvatarChangeCostumeReq = proto.into();
            Ok((
                crate::normal::AvatarChangeCostumeReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::PlayerEnterSceneNotify::CMD_ID => {
            let proto = crate::client::PlayerEnterSceneNotify::decode(body)?;
            let proto: crate::normal::PlayerEnterSceneNotify = proto.into();
            Ok((
                crate::normal::PlayerEnterSceneNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::PathfindingEnterSceneRsp::CMD_ID => {
            let proto = crate::client::PathfindingEnterSceneRsp::decode(body)?;
            let proto: crate::normal::PathfindingEnterSceneRsp = proto.into();
            Ok((
                crate::normal::PathfindingEnterSceneRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::EnterSceneDoneReq::CMD_ID => {
            let proto = crate::client::EnterSceneDoneReq::decode(body)?;
            let proto: crate::normal::EnterSceneDoneReq = proto.into();
            Ok((
                crate::normal::EnterSceneDoneReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::ServerTimeNotify::CMD_ID => {
            let proto = crate::client::ServerTimeNotify::decode(body)?;
            let proto: crate::normal::ServerTimeNotify = proto.into();
            Ok((
                crate::normal::ServerTimeNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::GetAreaExplorePointReq::CMD_ID => {
            let proto = crate::client::GetAreaExplorePointReq::decode(body)?;
            let proto: crate::normal::GetAreaExplorePointReq = proto.into();
            Ok((
                crate::normal::GetAreaExplorePointReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::ChangeAvatarRsp::CMD_ID => {
            let proto = crate::client::ChangeAvatarRsp::decode(body)?;
            let proto: crate::normal::ChangeAvatarRsp = proto.into();
            Ok((
                crate::normal::ChangeAvatarRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::AvatarWearFlycloakReq::CMD_ID => {
            let proto = crate::client::AvatarWearFlycloakReq::decode(body)?;
            let proto: crate::normal::AvatarWearFlycloakReq = proto.into();
            Ok((
                crate::normal::AvatarWearFlycloakReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::SetUpAvatarTeamRsp::CMD_ID => {
            let proto = crate::client::SetUpAvatarTeamRsp::decode(body)?;
            let proto: crate::normal::SetUpAvatarTeamRsp = proto.into();
            Ok((
                crate::normal::SetUpAvatarTeamRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::client::AvatarChangeCostumeRsp::CMD_ID => {
            let proto = crate::client::AvatarChangeCostumeRsp::decode(body)?;
            let proto: crate::normal::AvatarChangeCostumeRsp = proto.into();
            Ok((
                crate::normal::AvatarChangeCostumeRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        not_found => Err(ProtocolConversionError::NotFound(not_found)),
    }
}
#[allow(unused, warnings)]
pub fn normal_to_client(
    cmd_id: u16,
    body: &[u8],
) -> Result<(u16, Box<[u8]>), ProtocolConversionError> {
    use crate::packet::{read_cmd_id, NetPacket};
    use crate::{Protobuf, CmdID};
    match cmd_id {
        crate::normal::WearEquipRsp::CMD_ID => {
            let proto = crate::normal::WearEquipRsp::decode(body)?;
            let proto: crate::client::WearEquipRsp = proto.into();
            Ok((
                crate::client::WearEquipRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::MarkMapRsp::CMD_ID => {
            let proto = crate::normal::MarkMapRsp::decode(body)?;
            let proto: crate::client::MarkMapRsp = proto.into();
            Ok((
                crate::client::MarkMapRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::EnterSceneDoneRsp::CMD_ID => {
            let proto = crate::normal::EnterSceneDoneRsp::decode(body)?;
            let proto: crate::client::EnterSceneDoneRsp = proto.into();
            Ok((
                crate::client::EnterSceneDoneRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::SyncScenePlayTeamEntityNotify::CMD_ID => {
            let proto = crate::normal::SyncScenePlayTeamEntityNotify::decode(body)?;
            let proto: crate::client::SyncScenePlayTeamEntityNotify = proto.into();
            Ok((
                crate::client::SyncScenePlayTeamEntityNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::WearEquipReq::CMD_ID => {
            let proto = crate::normal::WearEquipReq::decode(body)?;
            let proto: crate::client::WearEquipReq = proto.into();
            Ok((
                crate::client::WearEquipReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::EvtAvatarStandUpNotify::CMD_ID => {
            let proto = crate::normal::EvtAvatarStandUpNotify::decode(body)?;
            let proto: crate::client::EvtAvatarStandUpNotify = proto.into();
            Ok((
                crate::client::EvtAvatarStandUpNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::EnterSceneReadyReq::CMD_ID => {
            let proto = crate::normal::EnterSceneReadyReq::decode(body)?;
            let proto: crate::client::EnterSceneReadyReq = proto.into();
            Ok((
                crate::client::EnterSceneReadyReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::ClientSetGameTimeRsp::CMD_ID => {
            let proto = crate::normal::ClientSetGameTimeRsp::decode(body)?;
            let proto: crate::client::ClientSetGameTimeRsp = proto.into();
            Ok((
                crate::client::ClientSetGameTimeRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::PlayerGameTimeNotify::CMD_ID => {
            let proto = crate::normal::PlayerGameTimeNotify::decode(body)?;
            let proto: crate::client::PlayerGameTimeNotify = proto.into();
            Ok((
                crate::client::PlayerGameTimeNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::AvatarEquipChangeNotify::CMD_ID => {
            let proto = crate::normal::AvatarEquipChangeNotify::decode(body)?;
            let proto: crate::client::AvatarEquipChangeNotify = proto.into();
            Ok((
                crate::client::AvatarEquipChangeNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::GetScenePointRsp::CMD_ID => {
            let proto = crate::normal::GetScenePointRsp::decode(body)?;
            let proto: crate::client::GetScenePointRsp = proto.into();
            Ok((
                crate::client::GetScenePointRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::ClientLoadingCostumeVerificationNotify::CMD_ID => {
            let proto = crate::normal::ClientLoadingCostumeVerificationNotify::decode(
                body,
            )?;
            let proto: crate::client::ClientLoadingCostumeVerificationNotify = proto
                .into();
            Ok((
                crate::client::ClientLoadingCostumeVerificationNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::AvatarWearFlycloakRsp::CMD_ID => {
            let proto = crate::normal::AvatarWearFlycloakRsp::decode(body)?;
            let proto: crate::client::AvatarWearFlycloakRsp = proto.into();
            Ok((
                crate::client::AvatarWearFlycloakRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::ClientSetGameTimeReq::CMD_ID => {
            let proto = crate::normal::ClientSetGameTimeReq::decode(body)?;
            let proto: crate::client::ClientSetGameTimeReq = proto.into();
            Ok((
                crate::client::ClientSetGameTimeReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::AvatarDataNotify::CMD_ID => {
            let proto = crate::normal::AvatarDataNotify::decode(body)?;
            let proto: crate::client::AvatarDataNotify = proto.into();
            Ok((
                crate::client::AvatarDataNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::EvtAvatarLockChairReq::CMD_ID => {
            let proto = crate::normal::EvtAvatarLockChairReq::decode(body)?;
            let proto: crate::client::EvtAvatarLockChairReq = proto.into();
            Ok((
                crate::client::EvtAvatarLockChairReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::SceneEntityAppearNotify::CMD_ID => {
            let proto = crate::normal::SceneEntityAppearNotify::decode(body)?;
            let proto: crate::client::SceneEntityAppearNotify = proto.into();
            Ok((
                crate::client::SceneEntityAppearNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::PingReq::CMD_ID => {
            let proto = crate::normal::PingReq::decode(body)?;
            let proto: crate::client::PingReq = proto.into();
            Ok((
                crate::client::PingReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::UnionCmdNotify::CMD_ID => {
            let proto = crate::normal::UnionCmdNotify::decode(body)?;
            let proto: crate::client::UnionCmdNotify = proto.into();
            Ok((
                crate::client::UnionCmdNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::AvatarFlycloakChangeNotify::CMD_ID => {
            let proto = crate::normal::AvatarFlycloakChangeNotify::decode(body)?;
            let proto: crate::client::AvatarFlycloakChangeNotify = proto.into();
            Ok((
                crate::client::AvatarFlycloakChangeNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::SceneTimeNotify::CMD_ID => {
            let proto = crate::normal::SceneTimeNotify::decode(body)?;
            let proto: crate::client::SceneTimeNotify = proto.into();
            Ok((
                crate::client::SceneTimeNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::OpenStateUpdateNotify::CMD_ID => {
            let proto = crate::normal::OpenStateUpdateNotify::decode(body)?;
            let proto: crate::client::OpenStateUpdateNotify = proto.into();
            Ok((
                crate::client::OpenStateUpdateNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::AvatarChangeTraceEffectReq::CMD_ID => {
            let proto = crate::normal::AvatarChangeTraceEffectReq::decode(body)?;
            let proto: crate::client::AvatarChangeTraceEffectReq = proto.into();
            Ok((
                crate::client::AvatarChangeTraceEffectReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::QueryPathReq::CMD_ID => {
            let proto = crate::normal::QueryPathReq::decode(body)?;
            let proto: crate::client::QueryPathReq = proto.into();
            Ok((
                crate::client::QueryPathReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::EvtBeingHitNotify::CMD_ID => {
            let proto = crate::normal::EvtBeingHitNotify::decode(body)?;
            let proto: crate::client::EvtBeingHitNotify = proto.into();
            Ok((
                crate::client::EvtBeingHitNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::NpcTalkRsp::CMD_ID => {
            let proto = crate::normal::NpcTalkRsp::decode(body)?;
            let proto: crate::client::NpcTalkRsp = proto.into();
            Ok((
                crate::client::NpcTalkRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::LifeStateChangeNotify::CMD_ID => {
            let proto = crate::normal::LifeStateChangeNotify::decode(body)?;
            let proto: crate::client::LifeStateChangeNotify = proto.into();
            Ok((
                crate::client::LifeStateChangeNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::PathfindingEnterSceneReq::CMD_ID => {
            let proto = crate::normal::PathfindingEnterSceneReq::decode(body)?;
            let proto: crate::client::PathfindingEnterSceneReq = proto.into();
            Ok((
                crate::client::PathfindingEnterSceneReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::GetScenePointReq::CMD_ID => {
            let proto = crate::normal::GetScenePointReq::decode(body)?;
            let proto: crate::client::GetScenePointReq = proto.into();
            Ok((
                crate::client::GetScenePointReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::GetPlayerTokenRsp::CMD_ID => {
            let proto = crate::normal::GetPlayerTokenRsp::decode(body)?;
            let proto: crate::client::GetPlayerTokenRsp = proto.into();
            Ok((
                crate::client::GetPlayerTokenRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::ChangeAvatarReq::CMD_ID => {
            let proto = crate::normal::ChangeAvatarReq::decode(body)?;
            let proto: crate::client::ChangeAvatarReq = proto.into();
            Ok((
                crate::client::ChangeAvatarReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::NpcTalkReq::CMD_ID => {
            let proto = crate::normal::NpcTalkReq::decode(body)?;
            let proto: crate::client::NpcTalkReq = proto.into();
            Ok((
                crate::client::NpcTalkReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::SceneInitFinishRsp::CMD_ID => {
            let proto = crate::normal::SceneInitFinishRsp::decode(body)?;
            let proto: crate::client::SceneInitFinishRsp = proto.into();
            Ok((
                crate::client::SceneInitFinishRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::PlayerDataNotify::CMD_ID => {
            let proto = crate::normal::PlayerDataNotify::decode(body)?;
            let proto: crate::client::PlayerDataNotify = proto.into();
            Ok((
                crate::client::PlayerDataNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::MarkMapReq::CMD_ID => {
            let proto = crate::normal::MarkMapReq::decode(body)?;
            let proto: crate::client::MarkMapReq = proto.into();
            Ok((
                crate::client::MarkMapReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::PlayerLoginReq::CMD_ID => {
            let proto = crate::normal::PlayerLoginReq::decode(body)?;
            let proto: crate::client::PlayerLoginReq = proto.into();
            Ok((
                crate::client::PlayerLoginReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::AvatarChangeTraceEffectRsp::CMD_ID => {
            let proto = crate::normal::AvatarChangeTraceEffectRsp::decode(body)?;
            let proto: crate::client::AvatarChangeTraceEffectRsp = proto.into();
            Ok((
                crate::client::AvatarChangeTraceEffectRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::EnterScenePeerNotify::CMD_ID => {
            let proto = crate::normal::EnterScenePeerNotify::decode(body)?;
            let proto: crate::client::EnterScenePeerNotify = proto.into();
            Ok((
                crate::client::EnterScenePeerNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::WindSeedClientNotify::CMD_ID => {
            let proto = crate::normal::WindSeedClientNotify::decode(body)?;
            let proto: crate::client::WindSeedClientNotify = proto.into();
            Ok((
                crate::client::WindSeedClientNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::GetSceneAreaRsp::CMD_ID => {
            let proto = crate::normal::GetSceneAreaRsp::decode(body)?;
            let proto: crate::client::GetSceneAreaRsp = proto.into();
            Ok((
                crate::client::GetSceneAreaRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::AvatarChangeCostumeNotify::CMD_ID => {
            let proto = crate::normal::AvatarChangeCostumeNotify::decode(body)?;
            let proto: crate::client::AvatarChangeCostumeNotify = proto.into();
            Ok((
                crate::client::AvatarChangeCostumeNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::EvtAvatarLockChairRsp::CMD_ID => {
            let proto = crate::normal::EvtAvatarLockChairRsp::decode(body)?;
            let proto: crate::client::EvtAvatarLockChairRsp = proto.into();
            Ok((
                crate::client::EvtAvatarLockChairRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::EnterSceneReadyRsp::CMD_ID => {
            let proto = crate::normal::EnterSceneReadyRsp::decode(body)?;
            let proto: crate::client::EnterSceneReadyRsp = proto.into();
            Ok((
                crate::client::EnterSceneReadyRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::PlayerNormalLuaShellNotify::CMD_ID => {
            let proto = crate::normal::PlayerNormalLuaShellNotify::decode(body)?;
            let proto: crate::client::PlayerNormalLuaShellNotify = proto.into();
            Ok((
                crate::client::PlayerNormalLuaShellNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::PlayerStoreNotify::CMD_ID => {
            let proto = crate::normal::PlayerStoreNotify::decode(body)?;
            let proto: crate::client::PlayerStoreNotify = proto.into();
            Ok((
                crate::client::PlayerStoreNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::SceneInitFinishReq::CMD_ID => {
            let proto = crate::normal::SceneInitFinishReq::decode(body)?;
            let proto: crate::client::SceneInitFinishReq = proto.into();
            Ok((
                crate::client::SceneInitFinishReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::EntityFightPropUpdateNotify::CMD_ID => {
            let proto = crate::normal::EntityFightPropUpdateNotify::decode(body)?;
            let proto: crate::client::EntityFightPropUpdateNotify = proto.into();
            Ok((
                crate::client::EntityFightPropUpdateNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::SetUpAvatarTeamReq::CMD_ID => {
            let proto = crate::normal::SetUpAvatarTeamReq::decode(body)?;
            let proto: crate::client::SetUpAvatarTeamReq = proto.into();
            Ok((
                crate::client::SetUpAvatarTeamReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::SceneForceUnlockNotify::CMD_ID => {
            let proto = crate::normal::SceneForceUnlockNotify::decode(body)?;
            let proto: crate::client::SceneForceUnlockNotify = proto.into();
            Ok((
                crate::client::SceneForceUnlockNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::HomeAvatarCostumeChangeNotify::CMD_ID => {
            let proto = crate::normal::HomeAvatarCostumeChangeNotify::decode(body)?;
            let proto: crate::client::HomeAvatarCostumeChangeNotify = proto.into();
            Ok((
                crate::client::HomeAvatarCostumeChangeNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::ScenePointUnlockNotify::CMD_ID => {
            let proto = crate::normal::ScenePointUnlockNotify::decode(body)?;
            let proto: crate::client::ScenePointUnlockNotify = proto.into();
            Ok((
                crate::client::ScenePointUnlockNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::GetPlayerTokenReq::CMD_ID => {
            let proto = crate::normal::GetPlayerTokenReq::decode(body)?;
            let proto: crate::client::GetPlayerTokenReq = proto.into();
            Ok((
                crate::client::GetPlayerTokenReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::UnlockTransPointReq::CMD_ID => {
            let proto = crate::normal::UnlockTransPointReq::decode(body)?;
            let proto: crate::client::UnlockTransPointReq = proto.into();
            Ok((
                crate::client::UnlockTransPointReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::AvatarChangeTraceEffectNotify::CMD_ID => {
            let proto = crate::normal::AvatarChangeTraceEffectNotify::decode(body)?;
            let proto: crate::client::AvatarChangeTraceEffectNotify = proto.into();
            Ok((
                crate::client::AvatarChangeTraceEffectNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::AvatarFightPropUpdateNotify::CMD_ID => {
            let proto = crate::normal::AvatarFightPropUpdateNotify::decode(body)?;
            let proto: crate::client::AvatarFightPropUpdateNotify = proto.into();
            Ok((
                crate::client::AvatarFightPropUpdateNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::SceneTeamUpdateNotify::CMD_ID => {
            let proto = crate::normal::SceneTeamUpdateNotify::decode(body)?;
            let proto: crate::client::SceneTeamUpdateNotify = proto.into();
            Ok((
                crate::client::SceneTeamUpdateNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::CombatInvocationsNotify::CMD_ID => {
            let proto = crate::normal::CombatInvocationsNotify::decode(body)?;
            let proto: crate::client::CombatInvocationsNotify = proto.into();
            Ok((
                crate::client::CombatInvocationsNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::QueryPathRsp::CMD_ID => {
            let proto = crate::normal::QueryPathRsp::decode(body)?;
            let proto: crate::client::QueryPathRsp = proto.into();
            Ok((
                crate::client::QueryPathRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::AvatarTeamUpdateNotify::CMD_ID => {
            let proto = crate::normal::AvatarTeamUpdateNotify::decode(body)?;
            let proto: crate::client::AvatarTeamUpdateNotify = proto.into();
            Ok((
                crate::client::AvatarTeamUpdateNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::PlayerLoginRsp::CMD_ID => {
            let proto = crate::normal::PlayerLoginRsp::decode(body)?;
            let proto: crate::client::PlayerLoginRsp = proto.into();
            Ok((
                crate::client::PlayerLoginRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::GetAreaExplorePointRsp::CMD_ID => {
            let proto = crate::normal::GetAreaExplorePointRsp::decode(body)?;
            let proto: crate::client::GetAreaExplorePointRsp = proto.into();
            Ok((
                crate::client::GetAreaExplorePointRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::GetSceneAreaReq::CMD_ID => {
            let proto = crate::normal::GetSceneAreaReq::decode(body)?;
            let proto: crate::client::GetSceneAreaReq = proto.into();
            Ok((
                crate::client::GetSceneAreaReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::PingRsp::CMD_ID => {
            let proto = crate::normal::PingRsp::decode(body)?;
            let proto: crate::client::PingRsp = proto.into();
            Ok((
                crate::client::PingRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::SceneEntityDisappearNotify::CMD_ID => {
            let proto = crate::normal::SceneEntityDisappearNotify::decode(body)?;
            let proto: crate::client::SceneEntityDisappearNotify = proto.into();
            Ok((
                crate::client::SceneEntityDisappearNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::PostEnterSceneRsp::CMD_ID => {
            let proto = crate::normal::PostEnterSceneRsp::decode(body)?;
            let proto: crate::client::PostEnterSceneRsp = proto.into();
            Ok((
                crate::client::PostEnterSceneRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::PostEnterSceneReq::CMD_ID => {
            let proto = crate::normal::PostEnterSceneReq::decode(body)?;
            let proto: crate::client::PostEnterSceneReq = proto.into();
            Ok((
                crate::client::PostEnterSceneReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::PlayerEnterSceneInfoNotify::CMD_ID => {
            let proto = crate::normal::PlayerEnterSceneInfoNotify::decode(body)?;
            let proto: crate::client::PlayerEnterSceneInfoNotify = proto.into();
            Ok((
                crate::client::PlayerEnterSceneInfoNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::EntityFightPropNotify::CMD_ID => {
            let proto = crate::normal::EntityFightPropNotify::decode(body)?;
            let proto: crate::client::EntityFightPropNotify = proto.into();
            Ok((
                crate::client::EntityFightPropNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::AvatarChangeCostumeReq::CMD_ID => {
            let proto = crate::normal::AvatarChangeCostumeReq::decode(body)?;
            let proto: crate::client::AvatarChangeCostumeReq = proto.into();
            Ok((
                crate::client::AvatarChangeCostumeReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::PlayerEnterSceneNotify::CMD_ID => {
            let proto = crate::normal::PlayerEnterSceneNotify::decode(body)?;
            let proto: crate::client::PlayerEnterSceneNotify = proto.into();
            Ok((
                crate::client::PlayerEnterSceneNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::PathfindingEnterSceneRsp::CMD_ID => {
            let proto = crate::normal::PathfindingEnterSceneRsp::decode(body)?;
            let proto: crate::client::PathfindingEnterSceneRsp = proto.into();
            Ok((
                crate::client::PathfindingEnterSceneRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::EnterSceneDoneReq::CMD_ID => {
            let proto = crate::normal::EnterSceneDoneReq::decode(body)?;
            let proto: crate::client::EnterSceneDoneReq = proto.into();
            Ok((
                crate::client::EnterSceneDoneReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::ServerTimeNotify::CMD_ID => {
            let proto = crate::normal::ServerTimeNotify::decode(body)?;
            let proto: crate::client::ServerTimeNotify = proto.into();
            Ok((
                crate::client::ServerTimeNotify::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::GetAreaExplorePointReq::CMD_ID => {
            let proto = crate::normal::GetAreaExplorePointReq::decode(body)?;
            let proto: crate::client::GetAreaExplorePointReq = proto.into();
            Ok((
                crate::client::GetAreaExplorePointReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::ChangeAvatarRsp::CMD_ID => {
            let proto = crate::normal::ChangeAvatarRsp::decode(body)?;
            let proto: crate::client::ChangeAvatarRsp = proto.into();
            Ok((
                crate::client::ChangeAvatarRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::AvatarWearFlycloakReq::CMD_ID => {
            let proto = crate::normal::AvatarWearFlycloakReq::decode(body)?;
            let proto: crate::client::AvatarWearFlycloakReq = proto.into();
            Ok((
                crate::client::AvatarWearFlycloakReq::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::SetUpAvatarTeamRsp::CMD_ID => {
            let proto = crate::normal::SetUpAvatarTeamRsp::decode(body)?;
            let proto: crate::client::SetUpAvatarTeamRsp = proto.into();
            Ok((
                crate::client::SetUpAvatarTeamRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        crate::normal::AvatarChangeCostumeRsp::CMD_ID => {
            let proto = crate::normal::AvatarChangeCostumeRsp::decode(body)?;
            let proto: crate::client::AvatarChangeCostumeRsp = proto.into();
            Ok((
                crate::client::AvatarChangeCostumeRsp::CMD_ID,
                proto.encode_to_vec().into_boxed_slice(),
            ))
        }
        not_found => Err(ProtocolConversionError::NotFound(not_found)),
    }
}
#[allow(unused, warnings)]
pub fn combat_invocation_client_to_normal(
    arg_type: crate::normal::CombatTypeArgument,
    combat_data: &[u8],
) -> Result<Box<[u8]>, ProtocolConversionError> {
    match arg_type {
        crate::normal::CombatTypeArgument::EvtBeingHit => {
            let proto = crate::client::EvtBeingHitInfo::decode(combat_data)?;
            let proto: crate::normal::EvtBeingHitInfo = proto.into();
            Ok(proto.encode_to_vec().into_boxed_slice())
        }
        crate::normal::CombatTypeArgument::EntityMove => {
            let proto = crate::client::EntityMoveInfo::decode(combat_data)?;
            let proto: crate::normal::EntityMoveInfo = proto.into();
            Ok(proto.encode_to_vec().into_boxed_slice())
        }
        not_found => Err(ProtocolConversionError::NotFoundCombatArgument(arg_type)),
    }
}
