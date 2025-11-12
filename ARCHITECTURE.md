# VxUtil 비디오 편집기 아키텍처 설명서

## 목차
1. [비디오 편집기란 무엇인가](#비디오-편집기란-무엇인가)
2. [전체 시스템 구조](#전체-시스템-구조)
3. [핵심 개념 설명](#핵심-개념-설명)
4. [데이터 흐름](#데이터-흐름)
5. [각 컴포넌트 상세 설명](#각-컴포넌트-상세-설명)
6. [렌더링 파이프라인](#렌더링-파이프라인)

---

## 비디오 편집기란 무엇인가

### NLE (Non-Linear Editor)
비디오 편집기는 "비선형 편집기"라고 불립니다. 이게 무슨 뜻일까요?

**선형 편집 (옛날 방식)**:
```
[테이프 1] → [테이프 2] → [테이프 3]
순서대로만 편집 가능, 중간에 수정하려면 처음부터 다시
```

**비선형 편집 (현대 방식)**:
```
[Timeline]
Track 1: [Clip A] [Clip B]    [Clip C]
Track 2:     [Clip D]      [Clip E]
Track 3: [Clip F]               [Clip G]

→ 언제든지 아무 클립이나 수정/이동/삭제 가능
```

### Premiere Pro vs After Effects
- **Premiere Pro**: 여러 비디오를 자르고 붙이는 "편집" 중심
- **After Effects**: 하나의 영상에 복잡한 "효과" 중심
- **VxUtil 목표**: 둘의 중간 (타임라인 편집 + 클립별 이펙트)

---

## 전체 시스템 구조

### 3계층 아키텍처

```
┌─────────────────────────────────────────────┐
│           vxutil-ui (사용자 인터페이스)        │
│  - Iced로 UI 렌더링                          │
│  - 사용자 입력 처리 (클릭, 드래그)             │
│  - 메시지 기반 상태 업데이트                   │
└──────────────────┬──────────────────────────┘
                   │ 사용
                   ↓
┌─────────────────────────────────────────────┐
│         vxutil-core (비즈니스 로직)           │
│  - Timeline, Track, Clip 데이터 모델         │
│  - Project, MediaLibrary 관리                │
│  - 순수한 데이터 구조 (IO 없음)               │
└──────────────────┬──────────────────────────┘
                   │ 사용
                   ↓
┌─────────────────────────────────────────────┐
│      vxutil-engine (미디어 처리 엔진)         │
│  - FFmpeg로 비디오 디코딩                    │
│  - WGPU로 GPU 가속 렌더링                    │
│  - 프레임 합성 (여러 클립 → 하나의 화면)       │
└─────────────────────────────────────────────┘
```

### 왜 이렇게 분리하나?
1. **테스트 용이**: UI 없이 core 로직만 테스트 가능
2. **컴파일 속도**: core 수정 시 UI 재컴파일 불필요
3. **재사용성**: engine을 CLI 툴에서도 사용 가능
4. **명확한 책임**: 각 crate가 하나의 역할만 담당

---

## 핵심 개념 설명

### 1. Project (프로젝트)
```
MyVideoProject/
├── project.json         ← 프로젝트 설정 파일
├── sequences/
│   └── main_sequence.json
└── cache/
    └── previews/
```

**역할**: 편집 작업의 최상위 컨테이너
- 프로젝트 설정 (프레임레이트, 해상도)
- 여러 Sequence를 포함
- 미디어 라이브러리

### 2. Sequence (시퀀스 = 타임라인)
```
Sequence "Main Timeline"
  Frame Rate: 30 fps
  Resolution: 1920x1080
  Duration: 00:05:30 (5분 30초)

  Tracks:
    Video Track 1: [Clip A] [Clip B] [Clip C]
    Video Track 2:     [Clip D]
    Audio Track 1: [Audio1]     [Audio2]
```

**역할**: 실제 편집 작업 공간
- 여러 트랙을 포함
- 프레임레이트와 해상도 정의
- 재생 위치(Playhead) 관리

**예시**:
- "인트로 시퀀스" (10초)
- "메인 컨텐츠 시퀀스" (5분)
- "아웃트로 시퀀스" (5초)

### 3. Track (트랙)
```
Video Track 1: [━━━━━━] [━━━] [━━━━━━━━]
Video Track 2: [━━] [━━━━━]
Video Track 3:         [━━━]
               ↓
           합성 순서: 3 → 2 → 1 (위에서 아래로)
```

**역할**: 클립들의 레이어
- **비디오 트랙**: 위에 있는 트랙이 앞에 표시됨 (Z-order)
- **오디오 트랙**: 모든 트랙의 소리가 믹싱됨
- Mute/Lock 기능

**비유**: 포토샵의 레이어와 비슷

### 4. Clip (클립)
```
원본 비디오: [0초─────30초─────60초]
                   ↓ (잘라냄)
Clip A: [30초────45초]
        ↓ (타임라인에 배치)
Timeline: [0초] [Clip A] [15초]
           ↑
        이 위치에 배치
```

**역할**: 타임라인에 배치된 미디어 조각
- **source_media**: 어떤 파일인가?
- **source_in/out**: 원본의 어느 부분? (30초~45초)
- **timeline_position**: 타임라인 어디에? (0초 위치)
- **speed**: 속도 (2.0 = 2배속)
- **effects**: 적용된 효과들

**실제 예시**:
```rust
Clip {
    source_media: "vacation.mp4",
    source_in: 30초,      // 원본 영상의 30초부터
    source_out: 45초,     // 45초까지 사용
    timeline_position: 10초,  // 타임라인의 10초 위치에
    speed: 1.5,           // 1.5배속으로
    effects: [Opacity(0.8), ColorCorrect { saturation: 1.2 }]
}
```

### 5. MediaItem (미디어 아이템)
```
MediaLibrary
├── video1.mp4 → MediaItem { duration: 120초, resolution: 1920x1080, ... }
├── audio1.mp3 → MediaItem { duration: 180초, sample_rate: 48000, ... }
└── image1.png → MediaItem { resolution: 3840x2160, ... }
```

**역할**: 원본 파일의 메타데이터 저장
- 파일 경로
- Duration, 해상도, 프레임레이트
- 코덱 정보
- 썸네일

**왜 필요한가?**: 매번 파일을 열어서 정보를 읽으면 느림

### 6. Effect (이펙트)
```
Clip [원본 프레임]
  ↓
Effect 1: Transform { x: 100, y: 50, scale: 1.2 }
  ↓
Effect 2: Opacity { alpha: 0.8 }
  ↓
Effect 3: ColorCorrect { saturation: 1.5 }
  ↓
[최종 프레임]
```

**역할**: 클립에 적용되는 시각적 변화
- Transform: 위치, 크기, 회전
- Opacity: 투명도
- Color: 색상 보정
- Blur, Sharpen 등

**적용 순서**: 위에서 아래로 순차 적용

---

## 데이터 흐름

### 편집 작업 흐름

```
1. 사용자가 비디오 파일 임포트
   ↓
2. FFmpeg가 메타데이터 추출
   MediaItem { duration: 120초, fps: 30, resolution: 1920x1080 }
   ↓
3. MediaLibrary에 저장
   ↓
4. 사용자가 타임라인에 드래그 앤 드롭
   ↓
5. Clip 생성 및 Track에 추가
   Clip { source: "video.mp4", timeline_pos: 10초, ... }
   ↓
6. UI 업데이트 (타임라인에 클립 표시)
```

### 재생 작업 흐름

```
사용자가 Play 버튼 클릭
   ↓
┌─────────────────────────────────────┐
│ 1. Playback Engine                  │
│    현재 시간: 10.5초                 │
│    "10.5초에 무엇을 보여줘야 하나?"  │
└──────────────┬──────────────────────┘
               ↓
┌─────────────────────────────────────┐
│ 2. Sequence 쿼리                     │
│    sequence.video_clips_at_time(10.5초) │
│    → [Clip A, Clip D]               │
└──────────────┬──────────────────────┘
               ↓
┌─────────────────────────────────────┐
│ 3. Frame Provider                   │
│    Clip A → 원본 비디오의 15초 프레임  │
│    Clip D → 원본 비디오의 3초 프레임   │
└──────────────┬──────────────────────┘
               ↓
┌─────────────────────────────────────┐
│ 4. FFmpeg Decoder                   │
│    파일 열기 → 해당 프레임 디코딩      │
│    → RGB 버퍼                        │
└──────────────┬──────────────────────┘
               ↓
┌─────────────────────────────────────┐
│ 5. Effect Processor                 │
│    각 클립에 이펙트 적용              │
│    Clip A: Opacity(0.8)             │
│    Clip D: Transform(scale: 1.2)    │
└──────────────┬──────────────────────┘
               ↓
┌─────────────────────────────────────┐
│ 6. Compositor                       │
│    여러 프레임을 하나로 합성          │
│    Track 순서대로 레이어링            │
│    알파 블렌딩                       │
└──────────────┬──────────────────────┘
               ↓
┌─────────────────────────────────────┐
│ 7. WGPU Renderer                    │
│    GPU로 최종 프레임 렌더링           │
│    → 화면에 출력                     │
└─────────────────────────────────────┘
```

### 내보내기 흐름

```
사용자가 Export 버튼 클릭
   ↓
타임라인 전체를 0초부터 끝까지 순회
   ↓
각 프레임마다:
  1. 위의 재생 흐름 반복
  2. 최종 프레임을 FFmpeg Encoder에 전달
  3. H.264로 압축
   ↓
최종 비디오 파일 저장
```

---

## 각 컴포넌트 상세 설명

### vxutil-core: Timeline 모듈

#### Timecode (타임코드)
```rust
// 시간을 표현하는 타입
Timecode(Duration)

// 사용 예시
Timecode::from_seconds(10.5)  // 10.5초
Timecode::from_frames(315, fps_30)  // 30fps 기준 315번째 프레임
```

**왜 필요한가?**:
- 프레임 단위로 정확한 편집 필요
- 다양한 프레임레이트 지원
- 타입 안전성 (실수로 픽셀과 혼동 방지)

#### TimeRange
```rust
TimeRange {
    start: Timecode(10초),
    duration: Duration(5초),
}
// → 10초부터 15초까지

// 사용처
- 클립이 차지하는 시간 범위
- 렌더링할 구간
- 선택 영역
```

#### Clip의 시간 계산

**기본 예시**:
```rust
Clip {
    source_in: 20초,
    source_out: 30초,        // 원본 10초 분량
    timeline_position: 5초,
    speed: 1.0,
}

// 타임라인에서 차지하는 구간: 5초 ~ 15초 (10초 동안)
```

**속도 변경 예시**:
```rust
Clip {
    source_in: 20초,
    source_out: 30초,        // 원본 10초 분량
    timeline_position: 5초,
    speed: 2.0,              // 2배속
}

// 타임라인에서 차지하는 구간: 5초 ~ 10초 (5초만 차지, 빠르게 재생)
```

**타임라인 시간 → 원본 시간 매핑**:
```
타임라인 7초에 뭘 보여줘야 하나?

1. 클립 내부 오프셋 계산: 7초 - 5초(position) = 2초
2. 속도 적용: 2초 × 2.0(speed) = 4초
3. 원본 위치 계산: 20초(source_in) + 4초 = 24초
→ 원본 영상의 24초 프레임을 보여줌
```

### vxutil-engine: FFmpeg 모듈

#### VideoDecoder
```rust
// 기능
1. 파일 열기
   decoder.open("video.mp4")

2. 메타데이터 읽기
   { duration: 120초, fps: 30, resolution: 1920x1080, codec: "H.264" }

3. 특정 프레임 추출
   decoder.seek_frame(150)  // 150번째 프레임으로 점프
   decoder.decode_next()    // RGB 버퍼 반환
```

**내부 동작**:
```
비디오 파일 구조:
[I-Frame] [P-Frame] [P-Frame] [I-Frame] [P-Frame] ...
   ↑                              ↑
GOP 시작                        GOP 시작

I-Frame: 전체 이미지
P-Frame: 이전 프레임과의 차이만 저장

150번 프레임 읽으려면:
1. 가장 가까운 I-Frame 찾기 (예: 120번)
2. 120번부터 150번까지 순차 디코딩
3. 150번 프레임 반환
```

#### Frame Cache
```rust
// 문제: 매번 디코딩하면 느림
프레임 읽기 → 50ms 소요
30fps → 33ms마다 새 프레임 필요
→ 실시간 재생 불가능!

// 해결: LRU 캐시
FrameCache {
    max_size: 100 frames,
    cache: HashMap<(file, frame_num), RgbBuffer>
}

// 동작
1. 프레임 요청
2. 캐시에 있나? → 즉시 반환 (0.1ms)
3. 없으면 디코딩 → 캐시에 저장 → 반환
4. 캐시 가득 차면 가장 오래된 프레임 삭제
```

### vxutil-engine: Compositor (합성기)

#### 합성 과정
```
입력: 타임라인 10초 시점의 모든 클립

Track 3: [Clip C - Logo PNG]     ← 최상위
Track 2: [Clip B - Overlay Video]
Track 1: [Clip A - Main Video]   ← 최하위

합성 알고리즘:
1. 빈 캔버스 생성 (1920x1080, 검은색)
2. Track 1 렌더링
   - Clip A의 프레임 가져오기
   - 이펙트 적용 (Transform, Color 등)
   - 캔버스에 그리기
3. Track 2 렌더링
   - Clip B의 프레임 가져오기
   - 이펙트 적용
   - 알파 블렌딩으로 캔버스에 합성
4. Track 3 렌더링
   - Clip C의 프레임 가져오기
   - 이펙트 적용
   - 알파 블렌딩으로 캔버스에 합성
5. 최종 프레임 반환

최종 결과 = A 위에 B, 그 위에 C
```

#### 알파 블렌딩
```rust
// 두 이미지를 투명도 고려해서 합성
fn blend(background: Rgb, foreground: Rgb, alpha: f32) -> Rgb {
    Rgb {
        r: background.r * (1.0 - alpha) + foreground.r * alpha,
        g: background.g * (1.0 - alpha) + foreground.g * alpha,
        b: background.b * (1.0 - alpha) + foreground.b * alpha,
    }
}

// 예시
배경(파란색): RGB(0, 0, 255)
전경(빨간색): RGB(255, 0, 0), alpha=0.5
결과: RGB(127, 0, 127)  // 보라색
```

### vxutil-ui: 사용자 인터페이스

#### Iced 아키텍처 (Elm-like)
```rust
// 1. State (상태)
struct VxUtil {
    project: Project,
    sequence: Sequence,
    playhead: Timecode,
    is_playing: bool,
}

// 2. Message (이벤트)
enum Message {
    Play,
    Pause,
    Seek(Timecode),
    AddClip { track_id, clip },
    RemoveClip { clip_id },
}

// 3. Update (상태 변경)
fn update(&mut self, message: Message) {
    match message {
        Message::Play => {
            self.is_playing = true;
            // Start playback timer
        }
        Message::Seek(time) => {
            self.playhead = time;
            // Update preview frame
        }
        ...
    }
}

// 4. View (화면 그리기)
fn view(&self) -> Element<Message> {
    column![
        toolbar(),
        row![
            media_panel(&self.project.media_library),
            preview_panel(&self.playhead),
        ],
        timeline_panel(&self.sequence),
    ]
}
```

#### UI 레이아웃
```
┌────────────────────────────────────────────────┐
│ Toolbar: [New] [Open] [Save] [Play] [Export]  │
├───────────┬────────────────────────────────────┤
│           │  Preview Window                    │
│  Media    │  ┌──────────────────────────┐     │
│  Browser  │  │                          │     │
│           │  │   [Video Frame]          │     │
│  ┌──────┐ │  │                          │     │
│  │Video1│ │  └──────────────────────────┘     │
│  ├──────┤ │  [◀ Seek Bar ▶]                   │
│  │Video2│ │                                    │
│  ├──────┤ │                                    │
│  │Audio1│ │                                    │
│  └──────┘ │                                    │
├───────────┴────────────────────────────────────┤
│  Timeline                                      │
│  ┌─────────────────────────────────────────┐  │
│  │ V1 ▓▓▓▓▓   ▓▓▓▓        ▓▓▓▓▓▓▓         │  │
│  │ V2      ▓▓▓▓    ▓▓▓▓▓                  │  │
│  │ A1 ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓          │  │
│  │    ▲                                    │  │
│  │ Playhead (현재 재생 위치)               │  │
│  └─────────────────────────────────────────┘  │
│  [00:00:10:15] [Zoom: 1x]                     │
└────────────────────────────────────────────────┘
```

---

## 렌더링 파이프라인

### GPU 가속 렌더링 (WGPU)

#### 왜 GPU를 사용하나?
```
CPU 렌더링 (느림):
1920 × 1080 = 2,073,600 픽셀
각 픽셀마다 계산 필요
→ CPU는 순차적으로 처리

GPU 렌더링 (빠름):
2,000개 코어가 동시에 처리
→ 1000배 이상 빠름
```

#### WGPU 파이프라인
```
1. Vertex Shader (정점 셰이더)
   화면에 사각형 그리기

2. Fragment Shader (픽셀 셰이더)
   각 픽셀의 색상 계산

예시: 투명도 이펙트
```
```rust
// fragment shader (WGSL)
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    return vec4<f32>(color.rgb, color.a * opacity);
    //                            ↑
    //                     투명도 적용
}
```

### 실시간 프리뷰 vs 최종 렌더링

```
┌─────────────────────────────────────────┐
│  실시간 프리뷰 (낮은 품질, 빠름)         │
│  - 해상도 낮춤 (1920→960)               │
│  - 프레임 건너뛰기 OK                   │
│  - 캐시 적극 활용                       │
│  - GPU 렌더링                           │
│  → 30fps 유지하는게 목표                │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│  최종 렌더링 (높은 품질, 느려도 OK)      │
│  - 원본 해상도 (1920×1080)              │
│  - 모든 프레임 렌더링                   │
│  - 고품질 인코딩 (높은 비트레이트)       │
│  - 시간 오래 걸려도 OK                  │
│  → 품질이 목표                          │
└─────────────────────────────────────────┘
```

---

## 최적화 전략

### 1. Frame Cache
- 최근 사용한 프레임 메모리에 보관
- LRU (Least Recently Used) 정책
- 메모리 제한: 예) 100프레임 = ~600MB

### 2. Lazy Loading
- 필요할 때만 파일 열기
- 메타데이터는 미리 로드

### 3. Multi-threading
```
Main Thread:     UI 처리
Decode Thread:   FFmpeg 디코딩
Render Thread:   WGPU 렌더링
Audio Thread:    오디오 재생

→ 병렬 처리로 성능 향상
```

### 4. Proxy Files
```
원본: 4K (3840×2160) → 디코딩 느림
프록시: HD (1280×720) → 편집 시 사용
최종 렌더링: 원본 4K 사용
```

---

## 요약: 전체 흐름 한눈에 보기

```
사용자 작업
    ↓
┌────────────────────────────────┐
│ vxutil-ui (Iced)               │
│ - 버튼 클릭 감지                │
│ - Message 생성                 │
│ - State 업데이트               │
└────────┬───────────────────────┘
         ↓
┌────────────────────────────────┐
│ vxutil-core                    │
│ - Sequence 데이터 수정         │
│ - Clip 추가/삭제               │
│ - 타임코드 계산                │
└────────┬───────────────────────┘
         ↓
┌────────────────────────────────┐
│ vxutil-engine                  │
│ - FFmpeg: 프레임 디코딩        │
│ - Compositor: 프레임 합성      │
│ - WGPU: GPU 렌더링             │
└────────┬───────────────────────┘
         ↓
    화면에 출력
```

---

## 다음 단계

이 문서를 읽고 나면:
1. ✅ 비디오 편집기의 전체 구조 이해
2. ✅ 각 컴포넌트의 역할 파악
3. ✅ 데이터가 어떻게 흐르는지 이해
4. ✅ 코드 구현 시 어디에 무엇을 넣어야 하는지 명확함

**이제 코드 구현을 시작할 준비가 되었습니다!**