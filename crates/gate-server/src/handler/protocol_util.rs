use mavuika_proto::{
    packet::{self, combat_invocation_client_to_normal},
    CmdID, CombatInvocationsNotify, CombatInvokeEntry, Protobuf, UnionCmd, UnionCmdNotify,
};
use tracing::{debug, instrument, warn};

#[instrument(skip_all)]
pub fn convert_union_cmd_notify_data(union_cmd_notify: UnionCmdNotify) -> UnionCmdNotify {
    let mut cmd_list = Vec::with_capacity(union_cmd_notify.cmd_list.len());
    for union_cmd in union_cmd_notify.cmd_list {
        if let Ok((sub_cmd_id, sub_cmd_body)) =
            packet::client_to_normal(union_cmd.message_id as u16, &union_cmd.body)
        {
            if sub_cmd_id == CombatInvocationsNotify::CMD_ID {
                if let Ok(combat_invocations_notify) =
                    CombatInvocationsNotify::decode(sub_cmd_body.as_ref())
                {
                    cmd_list.push(UnionCmd {
                        message_id: sub_cmd_id as u32,
                        body: convert_combat_invocations_notify_data(combat_invocations_notify)
                            .encode_to_vec(),
                    });
                }
            } else {
                cmd_list.push(UnionCmd {
                    message_id: sub_cmd_id as u32,
                    body: sub_cmd_body.into(),
                });
            }
        } else {
            warn!(
                "UnionCmdNotify: couldn't convert sub cmd with id {}",
                union_cmd.message_id
            );
        }
    }

    UnionCmdNotify { cmd_list }
}

pub fn convert_combat_invocations_notify_data(
    notify: CombatInvocationsNotify,
) -> CombatInvocationsNotify {
    let mut entries = Vec::with_capacity(notify.invoke_list.len());
    for entry in notify.invoke_list {
        if let Ok(data) =
            combat_invocation_client_to_normal(entry.argument_type(), &entry.combat_data)
        {
            debug!(
                "converted CombatInvokeEntry with ArgumentType {:?}",
                entry.argument_type()
            );

            entries.push(CombatInvokeEntry {
                combat_data: data.into(),
                argument_type: entry.argument_type,
                forward_type: entry.forward_type,
            });
        } else {
            warn!(
                "failed to convert CombatInvokeEntry with ArgumentType: {:?}",
                entry.argument_type()
            );
        }
    }

    CombatInvocationsNotify {
        invoke_list: entries,
    }
}
