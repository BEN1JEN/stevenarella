protocol_packet_ids!(
    handshake Handshaking {
        serverbound Serverbound {
            0x00 => Handshake
        }
        clientbound Clientbound {
        }
    }
    play Play {
        serverbound Serverbound {
            0x00 => TeleportConfirm
            0x01 => QueryBlockNBT
            0x02 => SetDifficulty
            //TODO 0x03 => ChatCommand
            0x04 => ChatMessage
            //TODO 0x05 => ChatPreview
            //TODO 0x06 => ClientCommand
            0x07 => ClientSettings_Filtering
            0x08 => TabComplete
            0x09 => ClickWindowButton
            0x0a => ClickWindow_State
            0x0b => CloseWindow
            0x0c => PluginMessageServerbound
            0x0d => EditBook_Pages
            0x0e => QueryEntityNBT
            0x0f => UseEntity_Sneakflag
            0x10 => GenerateStructure
            0x11 => KeepAliveServerbound_i64
            0x12 => LockDifficulty
            0x13 => PlayerPosition
            0x14 => PlayerPositionLook
            0x15 => PlayerLook
            0x16 => Player
            0x17 => VehicleMove
            0x18 => SteerBoat
            0x19 => PickItem
            0x1a => CraftRecipeRequest
            0x1b => ClientAbilities_u8
            0x1c => PlayerDigging
            0x1d => PlayerAction
            0x1e => SteerVehicle
            0x1f => WindowPong
            0x20 => SetDisplayedRecipe
            0x21 => SetRecipeBookState
            0x22 => NameItem
            0x23 => ResourcePackStatus
            0x24 => AdvancementTab
            0x25 => SelectTrade
            0x26 => SetBeaconEffect
            0x27 => HeldItemChange
            0x28 => UpdateCommandBlock
            0x29 => UpdateCommandBlockMinecart
            0x2a => CreativeInventoryAction
            0x2b => UpdateJigsawBlock_Joint
            0x2c => UpdateStructureBlock
            0x2d => SetSign
            0x2e => ArmSwing
            0x2f => SpectateTeleport
            0x30 => PlayerBlockPlacement_insideblock
            0x31 => UseItem
        }
        clientbound Clientbound {
            0x00 => SpawnObject_HeadYaw
            0x01 => SpawnExperienceOrb
            0x02 => SpawnPlayer_f64_NoMeta
            0x03 => Animation
            0x04 => Statistics
            0x05 => AcknowledgeBlockChange
            0x06 => BlockBreakAnimation
            0x07 => UpdateBlockEntity_VarInt
            0x08 => BlockAction
            0x09 => BlockChange_VarInt
            0x0a => BossBar
            0x0b => ServerDifficulty_Locked
            0x0c => ServerMessage_Sender // TODO: Chat Preview
            0x0d => ClearTitles
            0x0e => TabCompleteReply
            0x0f => DeclareCommands
            0x10 => WindowClose
            0x11 => WindowItems_StateCarry
            0x12 => WindowProperty
            0x13 => WindowSetSlot_State
            0x14 => SetCooldown
            //0x15 => ChatSuggestions // TODO
            0x16 => PluginMessageClientbound
            0x17 => NamedSoundEffect
            //0x18 => HideMessage // TODO
            0x19 => Disconnect
            0x1a => EntityAction
            0x1b => Explosion_VarInt
            0x1c => ChunkUnload
            0x1d => ChangeGameState
            0x1e => WindowOpenHorse
            0x1f => WorldBorderInit
            0x20 => KeepAliveClientbound_i64
            0x21 => ChunkData_AndLight
            0x22 => Effect
            0x23 => Particle_f64_VarInt
            0x24 => UpdateLight_Arrays
            0x25 => JoinGame_WorldNames_IsHard_SimDist_HasDeath
            0x26 => Maps
            0x27 => TradeList_WithRestock
            0x28 => EntityMove_i16
            0x29 => EntityLookAndMove_i16
            0x2a => EntityLook_VarInt
            0x2b => VehicleTeleport
            0x2c => OpenBook
            0x2d => WindowOpen_VarInt
            0x2e => SignEditorOpen
            0x2f => WindowPing
            0x30 => CraftRecipeResponse
            0x31 => PlayerAbilities
            //0x32 => MessageHeader // TODO
            //0x33 => PlayerChatMessage // TODO
            0x34 => CombatEventEnd
            0x35 => CombatEventEnter
            0x36 => CombatEventDeath
            0x37 => PlayerInfo
            0x38 => FacePlayer
            0x39 => TeleportPlayer_WithDismount
            0x3a => UnlockRecipes_WithBlastSmoker
            0x3b => EntityDestroy
            0x3c => EntityRemoveEffect_VarInt
            0x3d => ResourcePackSend_Prompt
            0x3e => Respawn_NBT
            0x3f => EntityHeadLook
            0x40 => MultiBlockChange_Packed
            0x41 => SelectAdvancementTab
            0x42 => ServerData
            0x43 => ActionBar
            0x44 => WorldBorderCenter
            0x45 => WorldBorderLerpSize
            0x46 => WorldBorderSize
            0x47 => WorldBorderWarningDelay
            0x48 => WorldBorderWarningReach
            0x49 => Camera
            0x4a => SetCurrentHotbarSlot
            0x4b => UpdateViewPosition
            0x4c => UpdateViewDistance
            0x4d => SpawnPosition_Angle
            //0x4e => SetDisplayChatPreview // TODO
            0x4f => ScoreboardDisplay
            0x50 => EntityMetadata
            0x51 => EntityAttach
            0x52 => EntityVelocity
            0x53 => EntityEquipment_Array
            0x54 => SetExperience
            0x55 => UpdateHealth
            0x56 => ScoreboardObjective
            0x57 => SetPassengers
            0x58 => Teams_VarInt
            0x59 => UpdateScore_VarInt
            0x5a => UpdateSimulationDistance
            0x5b => TitleSubtitle
            0x5c => TimeUpdate
            0x5d => Title
            0x5e => TitleTimes
            0x5f => EntitySoundEffect
            0x60 => SoundEffect
            0x61 => StopSound
            0x62 => SystemChatMessage
            0x63 => PlayerListHeaderFooter
            0x64 => NBTQueryResponse
            0x65 => CollectItem
            0x66 => EntityTeleport_f64
            0x67 => Advancements
            0x68 => EntityProperties_VarIntVarInt
            0x69 => EntityEffect_VarInt
            0x6a => DeclareRecipes
            0x6b => Tags_Nested
        }
    }
    login Login {
        serverbound Serverbound {
            0x00 => LoginStart_Sig
            0x01 => EncryptionResponse_Sig
            0x02 => LoginPluginResponse
        }
        clientbound Clientbound {
            0x00 => LoginDisconnect
            0x01 => EncryptionRequest
            0x02 => LoginSuccess_Sig
            0x03 => SetInitialCompression
            0x04 => LoginPluginRequest
        }
    }
    status Status {
        serverbound Serverbound {
            0x00 => StatusRequest
            0x01 => StatusPing
        }
        clientbound Clientbound {
            0x00 => StatusResponse
            0x01 => StatusPong
        }
    }
);
