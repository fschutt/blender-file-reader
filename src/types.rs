//! Defines the structs which the Blender file may contain
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use traits::{BlenderRead, BlenderWrite};
// use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use errors::Error as BlenderError;

/// Known struct that we can use
pub enum BlenderStruct {
    Link(BLink),
    LinkData(BLinkData),
    ListBase(BListBase),
    vec2s(Bvec2s),
    vec2i(Bvec2i),
    vec2f(Bvec2f),
    vec2d(Bvec2d),
    vec3i(Bvec3i),
    vec3f(Bvec3f),
    vec3d(Bvec3d),
    vec4i(Bvec4i),
    vec4f(Bvec4f),
    vec4d(Bvec4d),
    rcti(Brcti),
    rctf(Brctf),
    IDPropertyData(BIDPropertyData),
    IDProperty(BIDProperty),
    ID(BID),
    Library(BLibrary),
    PreviewImage(BPreviewImage),
    IpoDriver(BIpoDriver),
    IpoCurve(BIpoCurve),
    Ipo(BIpo),
    KeyBlock(BKeyBlock),
    Key(BKey),
    ScriptLink(BScriptLink),
    TextLine(BTextLine),
    TextMarker(BTextMarker),
    Text(BText),
    PackedFile(BPackedFile),
    Camera(BCamera),
    ImageUser(BImageUser),
    Image(BImage),
    MTex(BMTex),
    PluginTex(BPluginTex),
    CBData(BCBData),
    ColorBand(BColorBand),
    EnvMap(BEnvMap),
    Tex(BTex),
    TexMapping(BTexMapping),
    Lamp(BLamp),
    Wave(BWave),
    Material(BMaterial),
    VFont(BVFont),
    MetaElem(BMetaElem),
    MetaBall(BMetaBall),
    BezTriple(BBezTriple),
    BPoint(BBPoint),
    Nurb(BNurb),
    CharInfo(BCharInfo),
    TextBox(BTextBox),
    Curve(BCurve),
    Mesh(BMesh),
    TFace(BTFace),
    MFace(BMFace),
    MEdge(BMEdge),
    MDeformWeight(BMDeformWeight),
    MDeformVert(BMDeformVert),
    MVert(BMVert),
    MCol(BMCol),
    MTexPoly(BMTexPoly),
    MLoopUV(BMLoopUV),
    MLoopCol(BMLoopCol),
    MSticky(BMSticky),
    MSelect(BMSelect),
    MTFace(BMTFace),
    MFloatProperty(BMFloatProperty),
    MIntProperty(BMIntProperty),
    MStringProperty(BMStringProperty),
    OrigSpaceFace(BOrigSpaceFace),
    MultiresCol(BMultiresCol),
    MultiresColFace(BMultiresColFace),
    MultiresFace(BMultiresFace),
    MultiresEdge(BMultiresEdge),
    MultiresLevel(BMultiresLevel),
    Multires(BMultires),
    PartialVisibility(BPartialVisibility),
    ModifierData(BModifierData),
    SubsurfModifierData(BSubsurfModifierData),
    LatticeModifierData(BLatticeModifierData),
    CurveModifierData(BCurveModifierData),
    BuildModifierData(BBuildModifierData),
    MaskModifierData(BMaskModifierData),
    ArrayModifierData(BArrayModifierData),
    MirrorModifierData(BMirrorModifierData),
    EdgeSplitModifierData(BEdgeSplitModifierData),
    BevelModifierData(BBevelModifierData),
    BMeshModifierData(BBMeshModifierData),
    DisplaceModifierData(BDisplaceModifierData),
    UVProjectModifierData(BUVProjectModifierData),
    DecimateModifierData(BDecimateModifierData),
    SmoothModifierData(BSmoothModifierData),
    CastModifierData(BCastModifierData),
    WaveModifierData(BWaveModifierData),
    ArmatureModifierData(BArmatureModifierData),
    HookModifierData(BHookModifierData),
    SoftbodyModifierData(BSoftbodyModifierData),
    ClothModifierData(BClothModifierData),
    CollisionModifierData(BCollisionModifierData),
    BooleanModifierData(BBooleanModifierData),
    MDefInfluence(BMDefInfluence),
    MDefCell(BMDefCell),
    MeshDeformModifierData(BMeshDeformModifierData),
    ParticleSystemModifierData(BParticleSystemModifierData),
    ParticleInstanceModifierData(BParticleInstanceModifierData),
    ExplodeModifierData(BExplodeModifierData),
    FluidsimModifierData(BFluidsimModifierData),
    ShrinkwrapModifierData(BShrinkwrapModifierData),
    SimpleDeformModifierData(BSimpleDeformModifierData),
    Lattice(BLattice),
    bDeformGroup(BbDeformGroup),
    BoundBox(BBoundBox),
    Object(BObject),
    ObHook(BObHook),
    PartDeflect(BPartDeflect),
    PointCache(BPointCache),
    SBVertex(BSBVertex),
    BulletSoftBody(BBulletSoftBody),
    SoftBody(BSoftBody),
    FluidsimSettings(BFluidsimSettings),
    World(BWorld),
    Radio(BRadio),
    Base(BBase),
    AviCodecData(BAviCodecData),
    QuicktimeCodecData(BQuicktimeCodecData),
    FFMpegCodecData(BFFMpegCodecData),
    AudioData(BAudioData),
    SceneRenderLayer(BSceneRenderLayer),
    RenderData(BRenderData),
    RenderProfile(BRenderProfile),
    GameFraming(BGameFraming),
    TimeMarker(BTimeMarker),
    ImagePaintSettings(BImagePaintSettings),
    ParticleBrushData(BParticleBrushData),
    ParticleEditSettings(BParticleEditSettings),
    TransformOrientation(BTransformOrientation),
    ToolSettings(BToolSettings),
    BrushData(BBrushData),
    SculptData(BSculptData),
    Scene(BScene),
    BGpic(BBGpic),
    View3D(BView3D),
    View2D(BView2D),
    SpaceLink(BSpaceLink),
    SpaceInfo(BSpaceInfo),
    SpaceIpo(BSpaceIpo),
    SpaceButs(BSpaceButs),
    SpaceSeq(BSpaceSeq),
    SpaceFile(BSpaceFile),
    SpaceOops(BSpaceOops),
    SpaceImage(BSpaceImage),
    SpaceNla(BSpaceNla),
    SpaceText(BSpaceText),
    Script(BScript),
    SpaceScript(BSpaceScript),
    SpaceTime(BSpaceTime),
    SpaceNode(BSpaceNode),
    SpaceImaSel(BSpaceImaSel),
    ThemeUI(BThemeUI),
    ThemeSpace(BThemeSpace),
    ThemeWireColor(BThemeWireColor),
    bTheme(BbTheme),
    SolidLight(BSolidLight),
    UserDef(BUserDef),
    bScreen(BbScreen),
    ScrVert(BScrVert),
    ScrEdge(BScrEdge),
    Panel(BPanel),
    ScrArea(BScrArea),
    FileGlobal(BFileGlobal),
    StripElem(BStripElem),
    TStripElem(BTStripElem),
    StripCrop(BStripCrop),
    StripTransform(BStripTransform),
    StripColorBalance(BStripColorBalance),
    StripProxy(BStripProxy),
    Strip(BStrip),
    PluginSeq(BPluginSeq),
    Sequence(BSequence),
    MetaStack(BMetaStack),
    Editing(BEditing),
    WipeVars(BWipeVars),
    GlowVars(BGlowVars),
    TransformVars(BTransformVars),
    SolidColorVars(BSolidColorVars),
    SpeedControlVars(BSpeedControlVars),
    Effect(BEffect),
    BuildEff(BBuildEff),
    PartEff(BPartEff),
    WaveEff(BWaveEff),
    TreeStoreElem(BTreeStoreElem),
    TreeStore(BTreeStore),
    Oops(BOops),
    bProperty(BbProperty),
    bNearSensor(BbNearSensor),
    bMouseSensor(BbMouseSensor),
    bTouchSensor(BbTouchSensor),
    bKeyboardSensor(BbKeyboardSensor),
    bPropertySensor(BbPropertySensor),
    bActuatorSensor(BbActuatorSensor),
    bDelaySensor(BbDelaySensor),
    bCollisionSensor(BbCollisionSensor),
    bRadarSensor(BbRadarSensor),
    bRandomSensor(BbRandomSensor),
    bRaySensor(BbRaySensor),
    bMessageSensor(BbMessageSensor),
    bSensor(BbSensor),
    bJoystickSensor(BbJoystickSensor),
    bExpressionCont(BbExpressionCont),
    bPythonCont(BbPythonCont),
    bController(BbController),
    bAddObjectActuator(BbAddObjectActuator),
    bActionActuator(BbActionActuator),
    bSoundActuator(BbSoundActuator),
    bCDActuator(BbCDActuator),
    bEditObjectActuator(BbEditObjectActuator),
    bSceneActuator(BbSceneActuator),
    bPropertyActuator(BbPropertyActuator),
    bObjectActuator(BbObjectActuator),
    bIpoActuator(BbIpoActuator),
    bCameraActuator(BbCameraActuator),
    bConstraintActuator(BbConstraintActuator),
    bGroupActuator(BbGroupActuator),
    bRandomActuator(BbRandomActuator),
    bMessageActuator(BbMessageActuator),
    bGameActuator(BbGameActuator),
    bVisibilityActuator(BbVisibilityActuator),
    bTwoDFilterActuator(BbTwoDFilterActuator),
    bParentActuator(BbParentActuator),
    bStateActuator(BbStateActuator),
    bActuator(BbActuator),
    FreeCamera(BFreeCamera),
    bSound(BbSound),
    bSoundListener(BbSoundListener),
    SpaceSound(BSpaceSound),
    GroupObject(BGroupObject),
    Group(BGroup),
    Bone(BBone),
    bArmature(BbArmature),
    bPoseChannel(BbPoseChannel),
    bPose(BbPose),
    bActionGroup(BbActionGroup),
    bActionChannel(BbActionChannel),
    bAction(BbAction),
    SpaceAction(BSpaceAction),
    bConstraintChannel(BbConstraintChannel),
    bConstraint(BbConstraint),
    bConstraintTarget(BbConstraintTarget),
    bPythonConstraint(BbPythonConstraint),
    bKinematicConstraint(BbKinematicConstraint),
    bTrackToConstraint(BbTrackToConstraint),
    bRotateLikeConstraint(BbRotateLikeConstraint),
    bLocateLikeConstraint(BbLocateLikeConstraint),
    bMinMaxConstraint(BbMinMaxConstraint),
    bSizeLikeConstraint(BbSizeLikeConstraint),
    bActionConstraint(BbActionConstraint),
    bLockTrackConstraint(BbLockTrackConstraint),
    bFollowPathConstraint(BbFollowPathConstraint),
    bStretchToConstraint(BbStretchToConstraint),
    bRigidBodyJointConstraint(BbRigidBodyJointConstraint),
    bClampToConstraint(BbClampToConstraint),
    bChildOfConstraint(BbChildOfConstraint),
    bTransformConstraint(BbTransformConstraint),
    bLocLimitConstraint(BbLocLimitConstraint),
    bRotLimitConstraint(BbRotLimitConstraint),
    bSizeLimitConstraint(BbSizeLimitConstraint),
    bDistLimitConstraint(BbDistLimitConstraint),
    bActionModifier(BbActionModifier),
    bActionStrip(BbActionStrip),
    bNodeStack(BbNodeStack),
    bNodeSocket(BbNodeSocket),
    bNode(BbNode),
    bNodeLink(BbNodeLink),
    bNodeTree(BbNodeTree),
    NodeImageAnim(BNodeImageAnim),
    NodeBlurData(BNodeBlurData),
    NodeDBlurData(BNodeDBlurData),
    NodeBilateralBlurData(BNodeBilateralBlurData),
    NodeHueSat(BNodeHueSat),
    NodeImageFile(BNodeImageFile),
    NodeChroma(BNodeChroma),
    NodeTwoXYs(BNodeTwoXYs),
    NodeTwoFloats(BNodeTwoFloats),
    NodeGeometry(BNodeGeometry),
    NodeVertexCol(BNodeVertexCol),
    NodeDefocus(BNodeDefocus),
    NodeScriptDict(BNodeScriptDict),
    NodeGlare(BNodeGlare),
    NodeTonemap(BNodeTonemap),
    NodeLensDist(BNodeLensDist),
    CurveMapPoint(BCurveMapPoint),
    CurveMap(BCurveMap),
    CurveMapping(BCurveMapping),
    BrushClone(BBrushClone),
    Brush(BBrush),
    CustomDataLayer(BCustomDataLayer),
    CustomData(BCustomData),
    HairKey(BHairKey),
    ParticleKey(BParticleKey),
    ChildParticle(BChildParticle),
    ParticleData(BParticleData),
    ParticleSettings(BParticleSettings),
    ParticleSystem(BParticleSystem),
    ClothSimSettings(BClothSimSettings),
    ClothCollSettings(BClothCollSettings),
    bGPDspoint(BbGPDspoint),
    bGPDstroke(BbGPDstroke),
    bGPDframe(BbGPDframe),
    bGPDlayer(BbGPDlayer),
    bGPdata(BbGPdata),
}

impl Into<u16> for BlenderStruct {
    fn into(self) -> u16 {
        use self::BlenderStruct::*;
        match self {
            Link(_) => 0,
            LinkData(_) => 1,
            ListBase(_) => 2,
            vec2s(_) => 3,
            vec2i(_) => 4,
            vec2f(_) => 5,
            vec2d(_) => 6,
            vec3i(_) => 7,
            vec3f(_) => 8,
            vec3d(_) => 9,
            vec4i(_) => 10,
            vec4f(_) => 11,
            vec4d(_) => 12,
            rcti(_) => 13,
            rctf(_) => 14,
            IDPropertyData(_) => 15,
            IDProperty(_) => 16,
            ID(_) => 17,
            Library(_) => 18,
            PreviewImage(_) => 19,
            IpoDriver(_) => 20,
            IpoCurve(_) => 21,
            Ipo(_) => 22,
            KeyBlock(_) => 23,
            Key(_) => 24,
            ScriptLink(_) => 25,
            TextLine(_) => 26,
            TextMarker(_) => 27,
            Text(_) => 28,
            PackedFile(_) => 29,
            Camera(_) => 30,
            ImageUser(_) => 31,
            Image(_) => 32,
            MTex(_) => 33,
            PluginTex(_) => 34,
            CBData(_) => 35,
            ColorBand(_) => 36,
            EnvMap(_) => 37,
            Tex(_) => 38,
            TexMapping(_) => 39,
            Lamp(_) => 40,
            Wave(_) => 41,
            Material(_) => 42,
            VFont(_) => 43,
            MetaElem(_) => 44,
            MetaBall(_) => 45,
            BezTriple(_) => 46,
            BPoint(_) => 47,
            Nurb(_) => 48,
            CharInfo(_) => 49,
            TextBox (_) => 50,
            Curve (_) => 51,
            Mesh (_) => 52,
            TFace (_) => 53,
            MFace (_) => 54,
            MEdge (_) => 55,
            MDeformWeight (_) => 56,
            MDeformVert (_) => 57,
            MVert (_) => 58,
            MCol (_) => 59,
            MTexPoly (_) => 60,
            MLoopUV (_) => 61,
            MLoopCol (_) => 62,
            MSticky (_) => 63,
            MSelect (_) => 64,
            MTFace (_) => 65,
            MFloatProperty (_) => 66,
            MIntProperty (_) => 67,
            MStringProperty (_) => 68,
            OrigSpaceFace (_) => 69,
            MultiresCol (_) => 70,
            MultiresColFace (_) => 71,
            MultiresFace (_) => 72,
            MultiresEdge (_) => 73,
            MultiresLevel (_) => 74,
            Multires (_) => 75,
            PartialVisibility (_) => 76,
            ModifierData (_) => 77,
            SubsurfModifierData (_) => 78,
            LatticeModifierData (_) => 79,
            CurveModifierData (_) => 80,
            BuildModifierData (_) => 81,
            MaskModifierData (_) => 82,
            ArrayModifierData (_) => 83,
            MirrorModifierData (_) => 84,
            EdgeSplitModifierData (_) => 85,
            BevelModifierData (_) => 86,
            BMeshModifierData (_) => 87,
            DisplaceModifierData (_) => 88,
            UVProjectModifierData (_) => 89,
            DecimateModifierData (_) => 90,
            SmoothModifierData (_) => 91,
            CastModifierData (_) => 92,
            WaveModifierData (_) => 93,
            ArmatureModifierData (_) => 94,
            HookModifierData (_) => 95,
            SoftbodyModifierData (_) => 96,
            ClothModifierData (_) => 97,
            CollisionModifierData (_) => 98,
            BooleanModifierData (_) => 99,
            MDefInfluence (_) => 100,
            MDefCell (_) => 101,
            MeshDeformModifierData (_) => 102,
            ParticleSystemModifierData (_) => 103,
            ParticleInstanceModifierData (_) => 104,
            ExplodeModifierData (_) => 105,
            FluidsimModifierData (_) => 106,
            ShrinkwrapModifierData (_) => 107,
            SimpleDeformModifierData (_) => 108,
            Lattice (_) => 109,
            bDeformGroup (_) => 110,
            BoundBox (_) => 111,
            Object (_) => 112,
            ObHook (_) => 113,
            PartDeflect (_) => 114,
            PointCache (_) => 115,
            SBVertex (_) => 116,
            BulletSoftBody (_) => 117,
            SoftBody (_) => 118,
            FluidsimSettings (_) => 119,
            World (_) => 120,
            Radio (_) => 121,
            Base (_) => 122,
            AviCodecData (_) => 123,
            QuicktimeCodecData (_) => 124,
            FFMpegCodecData (_) => 125,
            AudioData (_) => 126,
            SceneRenderLayer (_) => 127,
            RenderData (_) => 128,
            RenderProfile (_) => 129,
            GameFraming (_) => 130,
            TimeMarker (_) => 131,
            ImagePaintSettings (_) => 132,
            ParticleBrushData (_) => 133,
            ParticleEditSettings (_) => 134,
            TransformOrientation (_) => 135,
            ToolSettings (_) => 136,
            BrushData (_) => 137,
            SculptData (_) => 138,
            Scene (_) => 139,
            BGpic (_) => 140,
            View3D (_) => 141,
            View2D (_) => 142,
            SpaceLink (_) => 143,
            SpaceInfo (_) => 144,
            SpaceIpo (_) => 145,
            SpaceButs (_) => 146,
            SpaceSeq (_) => 147,
            SpaceFile (_) => 148,
            SpaceOops (_) => 149,
            SpaceImage (_) => 150,
            SpaceNla (_) => 151,
            SpaceText (_) => 152,
            Script (_) => 153,
            SpaceScript (_) => 154,
            SpaceTime (_) => 155,
            SpaceNode (_) => 156,
            SpaceImaSel (_) => 157,
            ThemeUI (_) => 158,
            ThemeSpace (_) => 159,
            ThemeWireColor (_) => 160,
            bTheme (_) => 161,
            SolidLight (_) => 162,
            UserDef (_) => 163,
            bScreen (_) => 164,
            ScrVert (_) => 165,
            ScrEdge (_) => 166,
            Panel (_) => 167,
            ScrArea (_) => 168,
            FileGlobal (_) => 169,
            StripElem (_) => 170,
            TStripElem (_) => 171,
            StripCrop (_) => 172,
            StripTransform (_) => 173,
            StripColorBalance (_) => 174,
            StripProxy (_) => 175,
            Strip (_) => 176,
            PluginSeq (_) => 177,
            Sequence (_) => 178,
            MetaStack (_) => 179,
            Editing (_) => 180,
            WipeVars (_) => 181,
            GlowVars (_) => 182,
            TransformVars (_) => 183,
            SolidColorVars (_) => 184,
            SpeedControlVars (_) => 185,
            Effect (_) => 186,
            BuildEff (_) => 187,
            PartEff (_) => 188,
            WaveEff (_) => 189,
            TreeStoreElem (_) => 190,
            TreeStore (_) => 191,
            Oops (_) => 192,
            bProperty (_) => 193,
            bNearSensor (_) => 194,
            bMouseSensor (_) => 195,
            bTouchSensor (_) => 196,
            bKeyboardSensor (_) => 197,
            bPropertySensor (_) => 198,
            bActuatorSensor (_) => 199,
            bDelaySensor (_) => 200,
            bCollisionSensor (_) => 201,
            bRadarSensor (_) => 202,
            bRandomSensor (_) => 203,
            bRaySensor (_) => 204,
            bMessageSensor (_) => 205,
            bSensor (_) => 206,
            bJoystickSensor (_) => 207,
            bExpressionCont (_) => 208,
            bPythonCont (_) => 209,
            bController (_) => 210,
            bAddObjectActuator (_) => 211,
            bActionActuator (_) => 212,
            bSoundActuator (_) => 213,
            bCDActuator (_) => 214,
            bEditObjectActuator (_) => 215,
            bSceneActuator (_) => 216,
            bPropertyActuator (_) => 217,
            bObjectActuator (_) => 218,
            bIpoActuator (_) => 219,
            bCameraActuator (_) => 220,
            bConstraintActuator (_) => 221,
            bGroupActuator (_) => 222,
            bRandomActuator (_) => 223,
            bMessageActuator (_) => 224,
            bGameActuator (_) => 225,
            bVisibilityActuator (_) => 226,
            bTwoDFilterActuator (_) => 227,
            bParentActuator (_) => 228,
            bStateActuator (_) => 229,
            bActuator (_) => 230,
            FreeCamera (_) => 231,
            bSound (_) => 232,
            bSoundListener (_) => 233,
            SpaceSound (_) => 234,
            GroupObject (_) => 235,
            Group (_) => 236,
            Bone (_) => 237,
            bArmature (_) => 238,
            bPoseChannel (_) => 239,
            bPose (_) => 240,
            bActionGroup (_) => 241,
            bActionChannel (_) => 242,
            bAction (_) => 243,
            SpaceAction (_) => 244,
            bConstraintChannel (_) => 245,
            bConstraint (_) => 246,
            bConstraintTarget (_) => 247,
            bPythonConstraint (_) => 248,
            bKinematicConstraint (_) => 249,
            bTrackToConstraint (_) => 250,
            bRotateLikeConstraint (_) => 251,
            bLocateLikeConstraint (_) => 252,
            bMinMaxConstraint (_) => 253,
            bSizeLikeConstraint (_) => 254,
            bActionConstraint (_) => 255,
            bLockTrackConstraint (_) => 256,
            bFollowPathConstraint (_) => 257,
            bStretchToConstraint (_) => 258,
            bRigidBodyJointConstraint (_) => 259,
            bClampToConstraint (_) => 260,
            bChildOfConstraint (_) => 261,
            bTransformConstraint (_) => 262,
            bLocLimitConstraint (_) => 263,
            bRotLimitConstraint (_) => 264,
            bSizeLimitConstraint (_) => 265,
            bDistLimitConstraint (_) => 266,
            bActionModifier (_) => 267,
            bActionStrip (_) => 268,
            bNodeStack (_) => 269,
            bNodeSocket (_) => 270,
            bNode (_) => 271,
            bNodeLink (_) => 272,
            bNodeTree (_) => 273,
            NodeImageAnim (_) => 274,
            NodeBlurData (_) => 275,
            NodeDBlurData (_) => 276,
            NodeBilateralBlurData (_) => 277,
            NodeHueSat (_) => 278,
            NodeImageFile (_) => 279,
            NodeChroma (_) => 280,
            NodeTwoXYs (_) => 281,
            NodeTwoFloats (_) => 282,
            NodeGeometry (_) => 283,
            NodeVertexCol (_) => 284,
            NodeDefocus (_) => 285,
            NodeScriptDict (_) => 286,
            NodeGlare (_) => 287,
            NodeTonemap (_) => 288,
            NodeLensDist (_) => 289,
            CurveMapPoint (_) => 290,
            CurveMap (_) => 291,
            CurveMapping (_) => 292,
            BrushClone (_) => 293,
            Brush (_) => 294,
            CustomDataLayer (_) => 295,
            CustomData (_) => 296,
            HairKey (_) => 297,
            ParticleKey (_) => 298,
            ChildParticle (_) => 299,
            ParticleData (_) => 300,
            ParticleSettings (_) => 301,
            ParticleSystem (_) => 302,
            ClothSimSettings (_) => 303,
            ClothCollSettings (_) => 304,
            bGPDspoint (_) => 305,
            bGPDstroke (_) => 306,
            bGPDframe (_) => 307,
            bGPDlayer (_) => 308,
            bGPdata(_) => 309,
        }
    }
}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BLink {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BLinkData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BListBase {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct Bvec2s {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct Bvec2i {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct Bvec2f {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct Bvec2d {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct Bvec3i {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct Bvec3f {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct Bvec3d {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct Bvec4i {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct Bvec4f {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct Bvec4d {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct Brcti {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct Brctf {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BIDPropertyData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BIDProperty {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BID {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BLibrary {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BPreviewImage {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BIpoDriver {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BIpoCurve {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BIpo {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BKeyBlock {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BKey {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BScriptLink {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BTextLine {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BTextMarker {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BText {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BPackedFile {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BCamera {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BImageUser {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BImage {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMTex {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BPluginTex {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BCBData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BColorBand {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BEnvMap {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BTex {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BTexMapping {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BLamp {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BWave {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMaterial {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BVFont {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMetaElem {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMetaBall {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BBezTriple {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BBPoint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BNurb {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BCharInfo {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BTextBox {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BCurve {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMesh {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BTFace {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMFace {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMEdge {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMDeformWeight {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMDeformVert {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMVert {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMCol {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMTexPoly {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMLoopUV {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMLoopCol {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMSticky {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMSelect {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMTFace {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMFloatProperty {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMIntProperty {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMStringProperty {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BOrigSpaceFace {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMultiresCol {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMultiresColFace {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMultiresFace {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMultiresEdge {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMultiresLevel {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMultires {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BPartialVisibility {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSubsurfModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BLatticeModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BCurveModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BBuildModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMaskModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BArrayModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMirrorModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BEdgeSplitModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BBevelModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BBMeshModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BDisplaceModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BUVProjectModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BDecimateModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSmoothModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BCastModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BWaveModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BArmatureModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BHookModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSoftbodyModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BClothModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BCollisionModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BBooleanModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMDefInfluence {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMDefCell {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMeshDeformModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BParticleSystemModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BParticleInstanceModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BExplodeModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BFluidsimModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BShrinkwrapModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSimpleDeformModifierData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BLattice {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbDeformGroup {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BBoundBox {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BObject {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BObHook {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BPartDeflect {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BPointCache {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSBVertex {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BBulletSoftBody {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSoftBody {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BFluidsimSettings {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BWorld {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BRadio {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BBase {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BAviCodecData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BQuicktimeCodecData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BFFMpegCodecData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BAudioData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSceneRenderLayer {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BRenderData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BRenderProfile {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BGameFraming {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BTimeMarker {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BImagePaintSettings {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BParticleBrushData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BParticleEditSettings {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BTransformOrientation {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BToolSettings {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BBrushData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSculptData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BScene {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BBGpic {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BView3D {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BView2D {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSpaceLink {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSpaceInfo {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSpaceIpo {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSpaceButs {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSpaceSeq {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSpaceFile {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSpaceOops {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSpaceImage {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSpaceNla {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSpaceText {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BScript {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSpaceScript {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSpaceTime {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSpaceNode {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSpaceImaSel {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BThemeUI {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BThemeSpace {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BThemeWireColor {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbTheme {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSolidLight {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BUserDef {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbScreen {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BScrVert {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BScrEdge {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BPanel {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BScrArea {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BFileGlobal {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BStripElem {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BTStripElem {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BStripCrop {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BStripTransform {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BStripColorBalance {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BStripProxy {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BStrip {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BPluginSeq {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSequence {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BMetaStack {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BEditing {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BWipeVars {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BGlowVars {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BTransformVars {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSolidColorVars {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSpeedControlVars {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BEffect {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BBuildEff {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BPartEff {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BWaveEff {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BTreeStoreElem {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BTreeStore {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BOops {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbProperty {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbNearSensor {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbMouseSensor {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbTouchSensor {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbKeyboardSensor {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbPropertySensor {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbActuatorSensor {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbDelaySensor {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbCollisionSensor {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbRadarSensor {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbRandomSensor {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbRaySensor {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbMessageSensor {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbSensor {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbJoystickSensor {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbExpressionCont {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbPythonCont {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbController {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbAddObjectActuator {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbActionActuator {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbSoundActuator {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbCDActuator {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbEditObjectActuator {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbSceneActuator {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbPropertyActuator {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbObjectActuator {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbIpoActuator {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbCameraActuator {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbConstraintActuator {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbGroupActuator {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbRandomActuator {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbMessageActuator {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbGameActuator {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbVisibilityActuator {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbTwoDFilterActuator {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbParentActuator {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbStateActuator {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbActuator {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BFreeCamera {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbSound {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbSoundListener {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSpaceSound {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BGroupObject {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BGroup {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BBone {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbArmature {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbPoseChannel {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbPose {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbActionGroup {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbActionChannel {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbAction {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BSpaceAction {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbConstraintChannel {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbConstraint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbConstraintTarget {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbPythonConstraint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbKinematicConstraint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbTrackToConstraint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbRotateLikeConstraint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbLocateLikeConstraint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbMinMaxConstraint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbSizeLikeConstraint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbActionConstraint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbLockTrackConstraint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbFollowPathConstraint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbStretchToConstraint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbRigidBodyJointConstraint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbClampToConstraint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbChildOfConstraint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbTransformConstraint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbLocLimitConstraint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbRotLimitConstraint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbSizeLimitConstraint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbDistLimitConstraint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbActionModifier {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbActionStrip {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbNodeStack {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbNodeSocket {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbNode {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbNodeLink {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbNodeTree {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BNodeImageAnim {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BNodeBlurData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BNodeDBlurData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BNodeBilateralBlurData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BNodeHueSat {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BNodeImageFile {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BNodeChroma {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BNodeTwoXYs {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BNodeTwoFloats {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BNodeGeometry {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BNodeVertexCol {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BNodeDefocus {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BNodeScriptDict {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BNodeGlare {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BNodeTonemap {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BNodeLensDist {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BCurveMapPoint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BCurveMap {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BCurveMapping {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BBrushClone {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BBrush {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BCustomDataLayer {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BCustomData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BHairKey {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BParticleKey {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BChildParticle {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BParticleData {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BParticleSettings {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BParticleSystem {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BClothSimSettings {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BClothCollSettings {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbGPDspoint {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbGPDstroke {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbGPDframe {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbGPDlayer {

}

#[derive(Debug, Clone, BlenderRead, BlenderWrite)]
pub struct BbGPdata {

}

