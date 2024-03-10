# Cursor Hero

[![Visitors](https://api.visitorbadge.io/api/visitors?path=github.com%2FTeamDman%2FCursor-Hero&countColor=%23263759)](https://visitorbadge.io/status?path=github.com%2FTeamDman%2FCursor-Hero)

The plan: enhance LLM cooperation by leveraging Windows UI Automation.

**Video demo**

[![yt thumbnail](https://img.youtube.com/vi/t1PYks0UTL8/0.jpg)](https://youtu.be/t1PYks0UTL8)


If you have a DualShock controller, you will need something like [DS4Windows](https://ds4windows.dev/).

## Integrations

- [TeamDman/voice2text: A python program that types what you say if you're holding the hotkey (github.com)](https://github.com/teamdman/voice2text)
- [ollama/ollama: Get up and running with Llama 2, Mistral, and other large language models. (github.com)](https://github.com/ollama/ollama)

## Research Notes

### Inference

- [LM Studio](https://lmstudio.ai/)

### Fine Tuning

- [allenai/OLMo: Modeling, training, eval, and inference code for OLMo (github.com)](https://github.com/allenai/OLMo)
- [hiyouga/LLaMA-Factory: Unify Efficient Fine-tuning of 100+ LLMs (github.com)](https://github.com/hiyouga/LLaMA-Factory)
- [OpenAccess-AI-Collective/axolotl: Go ahead and axolotl questions (github.com)](https://github.com/OpenAccess-AI-Collective/axolotl)

### Vision

- [PTA-Text: A Text Only Click Model - Prompt image, it tells you where it would click](https://huggingface.co/AskUI/pta-text-0.1) ([demo](https://huggingface.co/spaces/AskUI/pta-text-v0.1))
- [Set-of-Mark Visual Prompting for GPT-4V](https://github.com/microsoft/SoM)
- [LLaVA](https://llava.hliu.cc/)
- [YOLOv9](https://github.com/WongKinYiu/yolov9)
- 

### RPA

- [OpenAdaptAI/OpenAdapt: AI-First Process Automation with Large Multimodal Models (LMMs)](https://github.com/OpenAdaptAI/OpenAdapt)
- [askui/askui: "What can be said can be solved-"](https://docs.askui.com/docs/api/Element-Descriptions/text)
- [ddupont808/GPT-4V-Act: AI agent using GPT-4V(ision) capable of using a mouse/keyboard to interact with web UI](https://www.reddit.com/r/MachineLearning/comments/17cy0j7/d_p_web_browsing_uibased_ai_agent_gpt4vact/?share_id=w5kHMEziP5LdHm_2NrlUc&rdt=49921)
- [TobiasNorlund/UI-Act: An AI agent for interacting with a computer using the graphical user interface](https://www.reddit.com/r/MachineLearning/comments/1765v6i/d_p_uibased_ai_agents_uiact/)
- [KillianLucas/open-interpreter: A natural language interface for computers](https://github.com/KillianLucas/open-interpreter)


### Windows UI Automation

- [Accessibility tools - AccEvent (Accessible Event Watcher) - Win32 apps | Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/winauto/accessible-event-watcher)
- [Accessibility tools - Inspect - Win32 apps | Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/winauto/inspect-objects)
- [Accessibility Insights](https://accessibilityinsights.io/)
- [Process Monitor - Sysinternals | Microsoft Learn](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon)
- [Navigation events for WebView2 apps - Microsoft Edge Developer documentation | Microsoft Learn](https://learn.microsoft.com/en-us/microsoft-edge/webview2/concepts/navigation-events)
- [(1) Building 25+ years of SysInternals: Exploring ZoomIt | BRK200H - YouTube](https://www.youtube.com/watch?v=W2bNgFrj3Iw)

- [c# - Getting icon of "modern" Windows app from a desktop application? - Stack Overflow](https://stackoverflow.com/questions/32122679/getting-icon-of-modern-windows-app-from-a-desktop-application)
- [How does Microsoft's "inspect.exe" application keep its window on top? - Microsoft Q&A === WS_EX_TOPMOST, UAC Bypass UI](https://learn.microsoft.com/en-us/answers/questions/1105704/how-does-microsofts-inspect-exe-application-keep-i)
  - ["Automatic dismissal of the start menu" and other crazyness - Windows-classic-samples/Samples/DesktopAutomationDismiss at 27ffb0811ca761741502feaefdb591aebf592193 - microsoft/Windows-classic-samples (github.com)](https://github.com/microsoft/Windows-classic-samples/tree/27ffb0811ca761741502feaefdb591aebf592193/Samples/DesktopAutomationDismiss#build-the-sample)
- [c# - UIAutomation won't retrieve children of an element - Stack Overflow](https://stackoverflow.com/questions/14187110/uiautomation-wont-retrieve-children-of-an-element)
- [comEle = \_AutomationClient.instance().dll.GetNextSiblingElement(self.Element) Hangs - Issue #30 - yinkaisheng/Python-UIAutomation-for-Windows (github.com)](https://github.com/yinkaisheng/Python-UIAutomation-for-Windows/issues/30)
- [microsoft/WinAppDriver: Windows Application Driver (github.com)](https://github.com/microsoft/WinAppDriver)
- [How to force UI automation tree refresh](https://stackoverflow.com/q/64120894/11141271)
  - mostly interested in the screen reader flag mentioned
    ```
    SystemParametersInfo( SPI_SETSCREENREADER, TRUE, NULL, SPIF_UPDATEINIFILE | SPIF_SENDCHANGE);
    PostMessage( HWND_BROADCAST, WM_WININICHANGE, SPI_SETSCREENREADER, 0);
    ```
- [UI Automation Fundamentals - .NET Framework | Microsoft Learn](https://learn.microsoft.com/en-us/dotnet/framework/ui-automation/ui-automation-fundamentals)
- [Understanding Threading Issues - Win32 apps | Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-threading)
- [UI Automation Threading Issues - .NET Framework | Microsoft Learn](https://learn.microsoft.com/en-us/dotnet/framework/ui-automation/ui-automation-threading-issues)
- [c# - System.Windows.Automation is extremely slow - Stack Overflow](https://stackoverflow.com/questions/41768046/system-windows-automation-is-extremely-slow)

### RL

- [stillonearth/bevy_rl](https://github.com/stillonearth/bevy_rl/blob/main/src/render.rs)
- [Saving RenderTarget image data to a file #5603](https://github.com/bevyengine/bevy/discussions/5603)
- [paulkre/bevy_image_export: Bevy plugin for rendering image sequences](https://github.com/paulkre/bevy_image_export)


### Prompting

- [guidance-ai/guidance: A guidance language for controlling large language models.](https://github.com/guidance-ai/guidance)
- [Eladlev/AutoPrompt: A framework for prompt tuning using Intent-based Prompt Calibration (github.com)](https://github.com/Eladlev/AutoPrompt)

### RAG, Tools and Actions

- [ACT-1: Transformer for Actions](https://www.adept.ai/blog/act-1)
- [LlamaIndex 🦙 v0.10.6](https://docs.llamaindex.ai/en/stable/)

### Sandboxing

- [copy/v86: x86 PC emulator and x86-to-wasm JIT, running in the browser](https://github.com/copy/v86) ([demo](https://copy.sh/v86/))

### Speech to Text

- [openai/whisper-large-v2: Hugging Face](https://huggingface.co/openai/whisper-large-v2)
- [m-bain/whisperX: Automatic Speech Recognition with Word-level Timestamps (& Diarization)](https://github.com/m-bain/whisperX)
- [SYSTRAN/faster-whisper: Faster Whisper transcription with CTranslate2](https://github.com/SYSTRAN/faster-whisper)
- [collabora/WhisperLive: A nearly-live implementation of OpenAI's Whisper](https://github.com/collabora/WhisperLive)
- [gaborvecsei/whisper-live-transcription: Live-Transcription (STT) with Whisper PoC (github.com)](https://github.com/gaborvecsei/whisper-live-transcription)
- [FL33TW00D/whisper-turbo: Cross-Platform, GPU Accelerated Whisper 🏎️ (github.com)](https://github.com/FL33TW00D/whisper-turbo)

### Multiplayer

- [Veilid: open-source, peer-to-peer, mobile-ﬁrst, networked application framework.](https://veilid.com/)

### Impl

- [beartype](https://beartype.readthedocs.io/en/latest/)
- [facebookresearch/torchdim: Named tensors with first-class dimensions for PyTorch](https://github.com/facebookresearch/torchdim)
- [Are we learning yet? A work-in-progress to catalog the state of machine learning in Rust](https://www.arewelearningyet.com/)
- [PyO3/pyo3: Rust bindings for the Python interpreter](https://github.com/PyO3/pyo3)

### Assets

- [Euphoric Heavy Regular](https://fontsgeek.com/fonts/Euphoric-Heavy-Regular)
- [Kenney Fonts](https://kenney.nl/assets/kenney-fonts)
- [Fira Mono - Google Fonts](https://fonts.google.com/specimen/Fira+Mono)
- [Xelu's FREE Controller Prompts (thoseawesomeguys.com)](https://thoseawesomeguys.com/prompts/)

### Bevy Plugins

- [janhohenheim/foxtrot: The all-in-one Bevy 3D game template for desktop. (github.com)](https://github.com/janhohenheim/foxtrot)
- [StarArawn/bevy\_ecs\_tilemap: A tilemap rendering crate for bevy which is more ECS friendly. (github.com)](https://github.com/StarArawn/bevy_ecs_tilemap)
- [djeedai/bevy\_hanabi: 🎆 Hanabi --- a GPU particle system plugin for the Bevy game engine. (github.com)](https://github.com/djeedai/bevy_hanabi)

### Game inspiration

- [The Last Clockwinder](https://store.steampowered.com/app/1755100/The_Last_Clockwinder/)

### Issues 👀

- [Transparent example not working - Issue #2502 - rust-windowing/winit (github.com)](https://github.com/rust-windowing/winit/issues/2502)
- 