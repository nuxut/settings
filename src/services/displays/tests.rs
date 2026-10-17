use super::*;

const LIST: &str = r#"{"v":1,"gen":12,"provider":"lua","pending":null,"primary":"Dell|U2720Q|ABC","displays":[
 {"id":"Dell|U2720Q|ABC","id_stable":true,"connector":"DP-1","make":"Dell Inc.","model":"DELL U2720Q","serial":"ABC",
  "description":"Dell Inc. DELL U2720Q ABC","physical_mm":[600,340],"connected":true,"enabled":true,
  "mode":{"w":3840,"h":2160,"mhz":59997},"pos":[0,0],"scale":1.5,"transform":0,"vrr":"off","vrr_capable":null,
  "mirror_of":null,"logical":[2560,1440],
  "resolutions":[{"w":3840,"h":2160,"rates_mhz":[59997,30000],"preferred":true,"scales":[1,1.25,1.5,2]}]},
 {"id":"LG|27GL|","id_stable":false,"connector":"HDMI-A-1","make":"LG","model":"27GL","serial":"",
  "description":"LG 27GL","connected":true,"enabled":true,
  "mode":{"w":1920,"h":1080,"mhz":144000},"pos":[2560,0],"scale":1.0,"transform":0,"vrr":"fullscreen",
  "mirror_of":null,"logical":[1920,1080],"resolutions":[],"future_field":7}]}"#;

fn layout() -> Layout {
    Layout::from_list(&List::parse(LIST).unwrap())
}

fn rect(l: &Layout, id: &str) -> Rect {
    l.get(id).unwrap().rect()
}

const DELL: &str = "Dell|U2720Q|ABC";
const LG: &str = "LG|27GL|";

#[test]
fn parses_contract_and_ignores_unknown_fields() {
    let list = List::parse(LIST).unwrap();
    assert_eq!(list.generation, 12);
    assert_eq!(list.displays.len(), 2);
    assert!(!list.displays[1].id_stable);
    assert_eq!(list.displays[1].vrr, Vrr::Fullscreen);
    let l = Layout::from_list(&list);
    assert_eq!(l.get(DELL).unwrap().scale_120, 180);
    assert_eq!(rect(&l, DELL), Rect { x: 0, y: 0, w: 2560, h: 1440 }, "shell'in `logical`'ı ile aynı hesap");
}

#[test]
fn rejects_out_of_contract_answers() {
    assert_eq!(List::parse(&" ".repeat(MAX_LIST_BYTES + 1)), Err(ParseError::TooLarge));
    assert_eq!(List::parse(r#"{"v":2,"gen":1,"displays":[]}"#), Err(ParseError::Version(2)));
    assert!(matches!(List::parse("{"), Err(ParseError::Json(_))));
    let long = LIST.replace("\"DP-1\"", &format!("\"{}\"", "x".repeat(65)));
    assert_eq!(List::parse(&long), Err(ParseError::Limit("dizgi")));
    let ctrl = LIST.replace("\"DP-1\"", "\"DP\\u0007\"");
    assert_eq!(List::parse(&ctrl), Err(ParseError::Limit("dizgi")));
    let zero = LIST.replace("\"w\":1920,\"h\":1080,\"mhz\":144000", "\"w\":0,\"h\":1080,\"mhz\":144000");
    assert_eq!(List::parse(&zero), Err(ParseError::Limit("kip")));
}

#[test]
fn logical_size_follows_scale_and_rotation() {
    let m = Mode { w: 3840, h: 2160, mhz: 60000 };
    assert_eq!(logical_size(m, 120, 0), (3840, 2160));
    assert_eq!(logical_size(m, 240, 0), (1920, 1080));
    assert_eq!(logical_size(m, 180, 1), (1440, 2560), "90° takas");
    assert_eq!(logical_size(m, 180, 4), (2560, 1440), "ayna döndürmesiz");
    assert!(scale_fits(m, 180));
    assert!(!scale_fits(Mode { w: 1366, h: 768, mhz: 60000 }, 150), "1366·120/150 tam değil");
}

#[test]
fn valid_layout_has_no_issues_and_serializes() {
    let l = layout();
    assert!(l.validate().is_empty(), "{:?}", l.validate());
    let json = l.apply_json().unwrap();
    assert!(json.starts_with(r#"{"v":1,"base_gen":12,"#));
    assert!(json.contains(r#""scale_120":180"#));
    assert!(json.contains(r#""vrr":"fullscreen""#));
    assert!(!json.contains('\n'));
}

#[test]
fn drag_snaps_edges_within_threshold() {
    let mut l = layout();
    // LG'yi Dell'in altına doğru sürükle; sol kenar 0'a, üst kenar 1440'a yapışır.
    l.drag_to(LG, 12, 1450, 16);
    assert_eq!(rect(&l, LG).x, 0);
    assert_eq!(rect(&l, LG).y, 1440);
    // Eşik dışında serbest.
    l.drag_to(LG, 300, 1700, 16);
    assert_eq!((rect(&l, LG).x, rect(&l, LG).y), (300, 1700));
}

#[test]
fn settle_moves_overlapping_display_to_nearest_free_edge() {
    let mut l = layout();
    l.get_mut(LG).unwrap().pos = [2000, 200]; // Dell'in içine girdi
    assert!(matches!(l.validate()[0], Issue::Overlap(..)));
    l.settle(LG);
    assert_eq!(rect(&l, LG), Rect { x: 2560, y: 200, w: 1920, h: 1080 }, "en yakın: sağ kenar, y korunur");
    assert!(l.validate().is_empty());
}

#[test]
fn settle_reattaches_detached_display() {
    let mut l = layout();
    l.get_mut(LG).unwrap().pos = [500, 3000];
    assert_eq!(l.validate(), vec![Issue::Disconnected]);
    l.settle(LG);
    assert_eq!(rect(&l, LG), Rect { x: 500, y: 1440, w: 1920, h: 1080 }, "alt kenara, x korunur");
    assert!(l.validate().is_empty());
}

#[test]
fn settle_keeps_valid_position_and_neighbours() {
    let mut l = layout();
    l.get_mut(LG).unwrap().pos = [2560, 360];
    l.settle(LG);
    assert_eq!(rect(&l, LG).y, 360, "geçerli konum değişmez");
    // Dell ölçeği 1'e inince büyür ve LG ile çakışır: LG değil, değişen (Dell) oturur.
    l.get_mut(DELL).unwrap().scale_120 = 120;
    l.settle(DELL);
    assert_eq!(rect(&l, LG).x, 2560, "komşu itilmez");
    assert!(l.validate().is_empty(), "{:?}", l.validate());
}

#[test]
fn corner_contact_counts_but_edge_is_preferred() {
    let a = Rect { x: 0, y: 0, w: 10, h: 10 };
    assert!(a.touches(&Rect { x: 10, y: 10, w: 5, h: 5 }), "köşe teması");
    assert!(!a.touches(&Rect { x: 11, y: 0, w: 5, h: 5 }));
    assert!(!a.touches(&Rect { x: 5, y: 5, w: 5, h: 5 }), "çakışma temas değil");
}

#[test]
fn validation_catches_shell_rejections() {
    let mut l = layout();
    l.get_mut(DELL).unwrap().enabled = false;
    l.get_mut(LG).unwrap().enabled = false;
    assert_eq!(l.validate(), vec![Issue::NoActiveDisplay]);

    let mut l = layout();
    l.get_mut(LG).unwrap().mirror_of = Some(DELL.into());
    assert!(l.validate().is_empty(), "yansıtma yerleşimde yer kaplamaz");
    l.get_mut(DELL).unwrap().mirror_of = Some(LG.into());
    assert!(l.validate().contains(&Issue::NoActiveDisplay), "birbirini yansıtma zinciri");

    let mut l = layout();
    l.get_mut(LG).unwrap().scale_120 = 150; // 1920·120/150 = 1536 tam, 1080·120/150 = 864 tam
    assert!(l.validate().is_empty());
    l.get_mut(LG).unwrap().scale_120 = 160; // 1080·120/160 = 810 tam, 1920·120/160 = 1440 tam
    assert!(l.validate().is_empty());
    l.get_mut(LG).unwrap().scale_120 = 175;
    assert_eq!(l.validate(), vec![Issue::InvalidScale(LG.into())]);
}

#[test]
fn normalize_moves_origin_to_zero() {
    let mut l = layout();
    l.get_mut(LG).unwrap().pos = [-1920, 100];
    l.normalize();
    assert_eq!(rect(&l, LG).x, 0);
    assert_eq!(rect(&l, LG).y, 100);
    assert_eq!(rect(&l, DELL).x, 1920);
    assert_eq!(rect(&l, DELL).y, 0, "en küçük y zaten 0");
}

#[test]
fn apply_json_respects_line_limit() {
    let mut l = layout();
    let proto = l.displays[0].clone();
    for i in 0..40 {
        l.displays.push(DisplayEdit { id: format!("{:<60}", i), ..proto.clone() });
    }
    assert!(l.apply_json().is_none());
}

#[test]
fn scale_choices_are_clean_divisors() {
    let m = Mode { w: 1920, h: 1080, mhz: 60000 };
    // 1920·120/s ve 1080·120/s tam: 1, 1.25, 1.333…, 1.5, 1.6, 2, 2.5, 3 (1.75 ve 2.25 değil).
    assert_eq!(scale_choices(None, m), vec![120, 150, 160, 180, 192, 240, 300, 360, 480]);
    let l = List::parse(LIST).unwrap();
    let dell = &l.displays[0];
    let res = dell.resolution(3840, 2160);
    assert_eq!(scale_choices(res, dell.mode), vec![120, 150, 180, 240], "shell'in önerisi");
    assert_eq!(nearest_scale(Mode { w: 1366, h: 768, mhz: 60000 }, 150), 120, "1366×768 yalnızca 1'e tam bölünür (önerilenler içinde)");
    assert!(scale_fits(m, nearest_scale(m, 175)));
}

#[test]
fn transform_splits_into_rotation_and_flip() {
    for t in 0..8u8 {
        assert_eq!(transform_of(rotation(t), flipped(t)), t);
    }
    assert_eq!((rotation(5), flipped(5)), (1, true));
}

#[test]
fn nearest_rate_prefers_closest_then_higher() {
    assert_eq!(nearest_rate(&[59997, 30000, 144000], 60000), Some(59997));
    assert_eq!(nearest_rate(&[50000, 70000], 60000), Some(70000));
    assert_eq!(nearest_rate(&[], 60000), None);
}

#[test]
fn mode_change_keeps_scale_clean_and_settles() {
    let mut l = layout();
    // Dell 3840×2160 %150 → 2560×1440: 2560/1.5 tam değil, en yakın temiz ölçeğe (1.6) iner.
    l.set_mode(DELL, Mode { w: 2560, h: 1440, mhz: 60000 });
    let d = l.get(DELL).unwrap();
    assert!(scale_fits(d.mode, d.scale_120), "ölçek temiz bölene indi: {}", d.scale_120);
    assert!(l.validate().is_empty(), "{:?}", l.validate());
    // Dell büyüdü (ölçek 1): LG itilmez, Dell oturur.
    let mut l = layout();
    l.set_scale(DELL, 120);
    assert_eq!(rect(&l, LG).x, 2560, "komşu itilmez");
    assert!(l.validate().is_empty(), "{:?}", l.validate());
    // 90° döndürme boyları takas eder, yine geçerli.
    l.set_transform(LG, 1);
    assert_eq!((rect(&l, LG).w, rect(&l, LG).h), (1080, 1920));
    assert!(l.validate().is_empty(), "{:?}", l.validate());
}

#[test]
fn enabling_mirroring_and_primary() {
    let mut l = layout();
    l.set_enabled(DELL, false);
    assert_eq!(l.primary, None, "kapanan birincil bırakılır");
    assert!(l.validate().is_empty(), "tek etkin monitör yeterli");
    l.set_enabled(DELL, true);
    assert!(l.validate().is_empty(), "açılan monitör çakışmadan oturur: {:?}", l.validate());

    let mut l = layout();
    l.set_mirror(DELL, Some(DELL.into()));
    assert_eq!(l.get(DELL).unwrap().mirror_of, None, "kendini yansıtamaz");
    l.set_mirror(DELL, Some(LG.into()));
    assert_eq!(l.primary, None, "yansıtan birincil olamaz");
    assert!(l.validate().is_empty());
    l.set_mirror(DELL, None);
    assert!(l.validate().is_empty(), "yansıtmadan çıkan oturur: {:?}", l.validate());
}

#[test]
fn change_detection_ignores_translation_and_rebase_keeps_edits() {
    let base = layout();
    let mut l = base.clone();
    for d in &mut l.displays {
        d.pos = [d.pos[0] + 100, d.pos[1] - 50];
    }
    assert!(!l.differs_from(&base), "yalnızca öteleme değişiklik değil");
    l.get_mut(LG).unwrap().vrr = Vrr::Off;
    assert!(l.differs_from(&base));

    let mut fresh = base.clone();
    fresh.base_gen = 13;
    assert!(l.rebase(&fresh), "aynı küme: düzenleme korunur");
    assert_eq!((l.base_gen, l.get(LG).unwrap().vrr), (13, Vrr::Off));
    fresh.displays.pop();
    assert!(!l.rebase(&fresh), "monitör çıktı: sıfırlanır");
    assert_eq!(l, fresh);
}
