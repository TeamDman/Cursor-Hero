# Research Notes

## Inference

- [LM Studio](https://lmstudio.ai/)
- [microsoft/monitors4codegen: Code and Data artifact for NeurIPS 2023 paper - "Monitor-Guided Decoding of Code LMs with Static Analysis of Repository Context". \`multispy\` is a lsp client library in Python intended to be used to build applications around language servers. (github.com)](https://github.com/microsoft/monitors4codegen)

## Fine Tuning

- [allenai/OLMo: Modeling, training, eval, and inference code for OLMo (github.com)](https://github.com/allenai/OLMo)
- [hiyouga/LLaMA-Factory: Unify Efficient Fine-tuning of 100+ LLMs (github.com)](https://github.com/hiyouga/LLaMA-Factory)
- [OpenAccess-AI-Collective/axolotl: Go ahead and axolotl questions (github.com)](https://github.com/OpenAccess-AI-Collective/axolotl)
- [LLaVA/docs/Finetune\_Custom\_Data.md at main - haotian-liu/LLaVA (github.com)](https://github.com/haotian-liu/LLaVA/blob/main/docs/Finetune_Custom_Data.md)


## Vision

- [PTA-Text: A Text Only Click Model - Prompt image, it tells you where it would click](https://huggingface.co/AskUI/pta-text-0.1) ([demo](https://huggingface.co/spaces/AskUI/pta-text-v0.1))
- [Set-of-Mark Visual Prompting for GPT-4V](https://github.com/microsoft/SoM)
- [LLaVA](https://llava.hliu.cc/)
- [YOLOv9](https://github.com/WongKinYiu/yolov9)
- [Ty on X: "Open Interpreter 0.2.1 is out! -  5X launch speed -  Local OSS model for GUI control -  Native control over Apple Mail, Calendar, Contacts, SMS -  New LLM-first web browser (powered by @perplexity\_ai) -  Profiles, Docker, Jupyter export... ↓ https://t.co/XU4gibTXWk" / X (twitter.com)](https://twitter.com/FieroTy/status/1767328066290987470)

## Image Processing

- [Template Matching by Correlation | Image Processing I - YouTube](https://www.youtube.com/watch?v=1_hwFc8PXVE)
- [Seam Carving Algorithm in Python | Liquid Resizing - YouTube](https://www.youtube.com/watch?v=g2FAonk6bss&feature=youtu.be)
- [Using Generative AI to Organize Video Game Screenshots (raymondcamden.com)](https://www.raymondcamden.com/2024/02/19/using-generative-ai-to-organize-video-game-screenshots)
- [Page dewarping (mzucker.github.io)](https://mzucker.github.io/2016/08/15/page-dewarping.html)

## RPA

- [OpenAdaptAI/OpenAdapt: AI-First Process Automation with Large Multimodal Models (LMMs)](https://github.com/OpenAdaptAI/OpenAdapt)
- [askui/askui: "What can be said can be solved-"](https://docs.askui.com/docs/api/Element-Descriptions/text)
- [ddupont808/GPT-4V-Act: AI agent using GPT-4V(ision) capable of using a mouse/keyboard to interact with web UI](https://www.reddit.com/r/MachineLearning/comments/17cy0j7/d_p_web_browsing_uibased_ai_agent_gpt4vact/?share_id=w5kHMEziP5LdHm_2NrlUc&rdt=49921)
- [TobiasNorlund/UI-Act: An AI agent for interacting with a computer using the graphical user interface](https://www.reddit.com/r/MachineLearning/comments/1765v6i/d_p_uibased_ai_agents_uiact/)
- [KillianLucas/open-interpreter: A natural language interface for computers](https://github.com/KillianLucas/open-interpreter)


## Windows UI Automation

- [Accessibility tools - AccEvent (Accessible Event Watcher) - Win32 apps | Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/winauto/accessible-event-watcher)
- [Accessibility tools - Inspect - Win32 apps | Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/winauto/inspect-objects)
- [Accessibility Insights](https://accessibilityinsights.io/)
- [Navigation events for WebView2 apps - Microsoft Edge Developer documentation | Microsoft Learn](https://learn.microsoft.com/en-us/microsoft-edge/webview2/concepts/navigation-events)
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
- [msdn-code-gallery-microsoft/Official Windows Platform Sample/UI Automation document content client sample at master - microsoftarchive/msdn-code-gallery-microsoft (github.com)](https://github.com/microsoftarchive/msdn-code-gallery-microsoft/tree/master/Official%20Windows%20Platform%20Sample/UI%20Automation%20document%20content%20client%20sample)
  - [Browse code samples | Microsoft Learn](https://learn.microsoft.com/en-us/samples/browse/)
- caching
  - [UIAutomation not catching all elements - Microsoft Q&A](https://learn.microsoft.com/en-us/answers/questions/545180/uiautomation-not-catching-all-elements)
  - [Caching UI Automation Properties and Control Patterns - Win32 apps | Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-cachingforclients)
  - [IUIAutomationElement::FindAllBuildCache (uiautomationclient.h) - Win32 apps | Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/uiautomationclient/nf-uiautomationclient-iuiautomationelement-findallbuildcache)
- [Writing a UI Automation Provider for a Win32-based Custom Control (codemag.com)](https://www.codemag.com/article/0810112/Writing-a-UI-Automation-Provider-for-a-Win32-based-Custom-Control)
- [Incorrect BoundingRectangle value - Issue #129 - yinkaisheng/Python-UIAutomation-for-Windows (github.com)](https://github.com/yinkaisheng/Python-UIAutomation-for-Windows/issues/129)
  - the UI Automation tree has bounding boxes for windows that include a padding and IDK why, found this issue which is probably unrelated lol
- [microsoft/accessibility-insights-windows: Accessibility Insights for Windows (github.com)](https://github.com/microsoft/accessibility-insights-windows)

### PowerAutomate

- [🤖 How to use Microsoft Power Automate Desktop - Full tutorial - YouTube](https://www.youtube.com/watch?v=IQ_KpBC8fwo&feature=youtu.be)

## Win32

- [How does Microsoft's "inspect.exe" application keep its window on top? - Microsoft Q&A === WS_EX_TOPMOST, UAC Bypass UI](https://learn.microsoft.com/en-us/answers/questions/1105704/how-does-microsofts-inspect-exe-application-keep-i)
  - ["Automatic dismissal of the start menu" and other crazyness - Windows-classic-samples/Samples/DesktopAutomationDismiss at 27ffb0811ca761741502feaefdb591aebf592193 - microsoft/Windows-classic-samples (github.com)](https://github.com/microsoft/Windows-classic-samples/tree/27ffb0811ca761741502feaefdb591aebf592193/Samples/DesktopAutomationDismiss#build-the-sample)
- `[Convert]::ToInt32("0x80070012", 16)` then ctrl+F win32::Foundation to find the error code
- [Process Hacker / Code / \[r6350\] /2.x/trunk (sourceforge.net)](https://sourceforge.net/p/processhacker/code/HEAD/tree/2.x/trunk/)
- [Process Monitor - Sysinternals | Microsoft Learn](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon)
- [(1) Building 25+ years of SysInternals: Exploring ZoomIt | BRK200H - YouTube](https://www.youtube.com/watch?v=W2bNgFrj3Iw)
- [c# - Getting icon of "modern" Windows app from a desktop application? - Stack Overflow](https://stackoverflow.com/questions/32122679/getting-icon-of-modern-windows-app-from-a-desktop-application)
- [DLL Export Viewer - view exported functions list in Windows DLL (nirsoft.net)](https://www.nirsoft.net/utils/dll_export_viewer.html)

### OneDrive

- [How can I check status of files/folders in OneDrive folder whether it is synced or not using c# or PowerShell?](https://stackoverflow.com/a/72857799/11141271)
- [rodneyviana/ODSyncService: OneDrive service/DLL for Sync State (github.com)](https://github.com/rodneyviana/ODSyncService)

## RL

- [stillonearth/bevy_rl](https://github.com/stillonearth/bevy_rl/blob/main/src/render.rs)
- [Saving RenderTarget image data to a file #5603](https://github.com/bevyengine/bevy/discussions/5603)
- [paulkre/bevy_image_export: Bevy plugin for rendering image sequences](https://github.com/paulkre/bevy_image_export)


## Prompting

- [guidance-ai/guidance: A guidance language for controlling large language models.](https://github.com/guidance-ai/guidance)
- [Eladlev/AutoPrompt: A framework for prompt tuning using Intent-based Prompt Calibration (github.com)](https://github.com/Eladlev/AutoPrompt)

## RAG, Tools and Actions

- [ACT-1: Transformer for Actions](https://www.adept.ai/blog/act-1)
- [LlamaIndex 🦙 v0.10.6](https://docs.llamaindex.ai/en/stable/)
- [Supercharge Your RAG with Contextualized Late Interactions (youtube.com)](https://www.youtube.com/watch?v=xTzUn3G9YA0)
  - [bclavie/RAGatouille](https://github.com/bclavie/RAGatouille)
- [PromtEngineer/YoutTube-Tutorial: This repo contains codes covered in the youtube tutorials. (github.com)](https://github.com/PromtEngineer/YoutTube-Tutorial)
  - [How Good is Phi-3-Mini for RAG, Routing, Agents (youtube.com)](https://www.youtube.com/watch?v=_4hFNzY1iMA)
  - [Advanced RAG with ColBERT in LangChain and LlamaIndex (youtube.com)](https://www.youtube.com/watch?v=kEgeegk9iqo)
  - [Supercharge Your RAG with Contextualized Late Interactions (youtube.com)](https://www.youtube.com/watch?v=xTzUn3G9YA0)
  - [Feature Test for Phi-3-mini-4k-instruct - LlamaIndex](https://docs.llamaindex.ai/en/latest/examples/benchmarks/phi-3-mini-4k-instruct/)

## Database

- [Quickstart | Get Started | EdgeDB Docs](https://docs.edgedb.com/get-started/quickstart)
- [bclavie/RAGatouille: Easily use and train state of the art late-interaction retrieval methods (ColBERT) in any RAG pipeline. Designed for modularity and ease-of-use, backed by research. (github.com)](https://github.com/bclavie/RAGatouille)


## Sandboxing

- [copy/v86: x86 PC emulator and x86-to-wasm JIT, running in the browser](https://github.com/copy/v86) ([demo](https://copy.sh/v86/))

## Speech to Text

- [openai/whisper-large-v2: Hugging Face](https://huggingface.co/openai/whisper-large-v2)
- [m-bain/whisperX: Automatic Speech Recognition with Word-level Timestamps (& Diarization)](https://github.com/m-bain/whisperX)
- [SYSTRAN/faster-whisper: Faster Whisper transcription with CTranslate2](https://github.com/SYSTRAN/faster-whisper)
- [collabora/WhisperLive: A nearly-live implementation of OpenAI's Whisper](https://github.com/collabora/WhisperLive)
- [gaborvecsei/whisper-live-transcription: Live-Transcription (STT) with Whisper PoC (github.com)](https://github.com/gaborvecsei/whisper-live-transcription)
- [FL33TW00D/whisper-turbo: Cross-Platform, GPU Accelerated Whisper 🏎️ (github.com)](https://github.com/FL33TW00D/whisper-turbo)

## Multiplayer

- [Veilid: open-source, peer-to-peer, mobile-ﬁrst, networked application framework.](https://veilid.com/)
- [stillonearth/bevy\_veilid: Build two-player turn-based p2p games with Bevy and Veilid (github.com)](https://github.com/stillonearth/bevy_veilid)

## Impl

- [beartype](https://beartype.readthedocs.io/en/latest/)
- [facebookresearch/torchdim: Named tensors with first-class dimensions for PyTorch](https://github.com/facebookresearch/torchdim)
- [Are we learning yet? A work-in-progress to catalog the state of machine learning in Rust](https://www.arewelearningyet.com/)
- [PyO3/pyo3: Rust bindings for the Python interpreter](https://github.com/PyO3/pyo3)

## Assets

- [Euphoric Heavy Regular](https://fontsgeek.com/fonts/Euphoric-Heavy-Regular)
- [Kenney Fonts](https://kenney.nl/assets/kenney-fonts)
- [Fira Mono - Google Fonts](https://fonts.google.com/specimen/Fira+Mono)
- [Xelu's FREE Controller Prompts (thoseawesomeguys.com)](https://thoseawesomeguys.com/prompts/)
- [\[DIVIDE BY ZERO\] Fonts : 1998-infinity (tom7.com)](http://fonts.tom7.com/fonts98.html)
- [\[DIVIDE BY ZERO\] Fonts : 1993-1997 (tom7.com)](http://fonts.tom7.com/fonts93.html)

## Bevy stuff

- [janhohenheim/foxtrot: The all-in-one Bevy 3D game template for desktop. (github.com)](https://github.com/janhohenheim/foxtrot)
- [StarArawn/bevy\_ecs\_tilemap: A tilemap rendering crate for bevy which is more ECS friendly. (github.com)](https://github.com/StarArawn/bevy_ecs_tilemap)
- [djeedai/bevy\_hanabi: 🎆 Hanabi --- a GPU particle system plugin for the Bevy game engine. (github.com)](https://github.com/djeedai/bevy_hanabi)
- [QueryLens in bevy::ecs::system - Rust (docs.rs)](https://docs.rs/bevy/latest/bevy/ecs/system/struct.QueryLens.html)
  - keywords for when I am trying to find this later: telescope, queryparams, systemparams
- [Entity-entity relations 🌈 - Issue #3742 - bevyengine/bevy (github.com)](https://github.com/bevyengine/bevy/issues/3742)
  - [Flecs: Relationships](https://www.flecs.dev/flecs/md_docs_2Relationships.html#symmetric-property)
  - [iiYese/aery: A plugin that enables a subset of entity relationship features for bevy (github.com)](https://github.com/iiYese/aery)
## Game inspiration

- [The Last Clockwinder](https://store.steampowered.com/app/1755100/The_Last_Clockwinder/)

## Issues 👀

- [Transparent example not working - Issue #2502 - rust-windowing/winit (github.com)](https://github.com/rust-windowing/winit/issues/2502)

## SDF

- [SDF arbitary 2D polygon (shadertoy.com)](https://www.shadertoy.com/view/WdSGRd)
- [Arbitrary Polygons, Blur/Border (shadertoy.com)](https://www.shadertoy.com/view/ctBXDK)

## Video making

- [Quickstart | Motion Canvas](https://motioncanvas.io/docs/quickstart/)
  - [examples/examples at master - motion-canvas/examples (github.com)](https://github.com/motion-canvas/examples/tree/master/examples)
  - [reviewing your motion canvas animations ![](https://img.youtube.com/vi/lY6D9x9qCt4/0.jpg)](https://www.youtube.com/watch?v=lY6D9x9qCt4)

## Art

- [LibreSprite/LibreSprite: Animated sprite editor & pixel art tool -- Fork of the last GPLv2 commit of Aseprite (github.com)](https://github.com/LibreSprite/LibreSprite)

## UI

- [linebender/vello: An experimental GPU compute-centric 2D renderer. (github.com)](https://github.com/linebender/vello)
  - [loopystudios/bevy\_vello: An integration to render with Vello in Bevy game engine. (github.com)](https://github.com/loopystudios/bevy_vello)

## Evolutionary Algorithms

- [bones-ai/rust-snake-ai: Training a Neural Network to play snake, uses genetic algorithm for neuro-evolution (github.com)](https://github.com/bones-ai/rust-snake-ai)

## Hot Reloading

- [Hot Reloading Rust --- for Fun and Faster Feedback Cycles | Robert Krahn](https://robert.kra.hn/posts/hot-reloading-rust/)
- [draivin/rust-hotswap: Easily hotswap functions in running executables (github.com)](https://github.com/draivin/rust-hotswap)
- [Hot Reloading Rust: Windows and Linux --- John Austin](https://johnaustin.io/articles/2022/hot-reloading-rust)

## Docs

- [tbillington/bevy\_game\_docs: Generate documentation for Bevy games (github.com)](https://github.com/tbillington/bevy_game_docs)
  - [bevy\_game\_docs/src/main.rs at main - tbillington/bevy\_game\_docs (github.com)](https://github.com/tbillington/bevy_game_docs/blob/main/src/main.rs)
  - [Motion Blur, Visualizations, and Beautiful Renders - This Week in Bevy Engine](https://youtu.be/M55pVZ6a8yk?t=286)

## Permissions

- [Open Policy Agent | Policy Language](https://www.openpolicyagent.org/docs/latest/policy-language/)

## Audio

- [Added Audio Input Support by bushrat011899 - Pull Request #10072 - bevyengine/bevy (github.com)](https://github.com/bevyengine/bevy/pull/10072)
- [VB-Audio VoiceMeeter Banana](https://vb-audio.com/Voicemeeter/banana.htm)
- [Virtual Audio Device/Card/Adapter - help - The Rust Programming Language Forum (rust-lang.org)](https://users.rust-lang.org/t/virtual-audio-device-card-adapter/58934)
- [c++ - How to create a virtual audio input device to simulate a microphone on windows? - Stack Overflow](https://stackoverflow.com/questions/74907682/how-to-create-a-virtual-audio-input-device-to-simulate-a-microphone-on-windows)
- [Windows-driver-samples/audio/simpleaudiosample at main - microsoft/Windows-driver-samples (github.com)](https://github.com/microsoft/Windows-driver-samples/tree/main/audio/simpleaudiosample)
- [wdmaudiodev Mailing List Archive (freelists.org)](https://www.freelists.org/archive/wdmaudiodev/)

## Animations

- [rive-app/rive-bevy (github.com)](https://github.com/rive-app/rive-bevy)
  - [How Duolingo Animates Its World Characters](https://blog.duolingo.com/world-character-visemes/)


## Models

- [mxbai-embed-large (ollama.com)](https://ollama.com/library/mxbai-embed-large)
- [starcoder2 (ollama.com)](https://ollama.com/library/starcoder2)
- [dolphincoder (ollama.com)](https://ollama.com/library/dolphincoder)
- [Function Calling in Ollama vs OpenAI (youtube.com)](https://www.youtube.com/watch?v=RXDWkiuXtG0&t=1s)

## Transparent windows not working

- [Bevy Transparent Window not working - Issue #10929 - bevyengine/bevy (github.com)](https://github.com/bevyengine/bevy/issues/10929)
- [Window transparency broken on Windows - Issue #7544 - bevyengine/bevy (github.com)](https://github.com/bevyengine/bevy/issues/7544)
- [Transparent example not working - Issue #2502 - rust-windowing/winit (github.com)](https://github.com/rust-windowing/winit/issues/2502)
- [Issues - rust-windowing/winit (github.com)](https://github.com/rust-windowing/winit/issues?q=is%3Aissue+is%3Aopen+transparency)
- [transparent example doesn't work because softbuffer doesn't support transparency - Issue #2960 - rust-windowing/winit (github.com)](https://github.com/rust-windowing/winit/issues/2960)
- [Transparent example not working - Issue #2502 - rust-windowing/winit (github.com)](https://github.com/rust-windowing/winit/issues/2502)
- [On Windows, improve support for undecorated windows by msiglreith - Pull Request #2419 - rust-windowing/winit (github.com)](https://github.com/rust-windowing/winit/pull/2419/files)
- [fixed window transparency by mgalos999 - Pull Request #2503 - rust-windowing/winit (github.com)](https://github.com/rust-windowing/winit/pull/2503)
- [rust-windowing/glutin: A low-level library for OpenGL context creation, written in pure Rust. (github.com)](https://github.com/rust-windowing/glutin)
- [Issues - rust-windowing/glutin (github.com)](https://github.com/rust-windowing/glutin/issues?q=is%3Aissue+transparent+is%3Aclosed)
- [X11 Window is always transparent with 0.30 - Issue #1559 - rust-windowing/glutin (github.com)](https://github.com/rust-windowing/glutin/issues/1559)
- [Transparency broken on X11 - Issue #1191 - rust-windowing/glutin (github.com)](https://github.com/rust-windowing/glutin/issues/1191)
- [Triangle example fails - cargo.toml problem? - Issue #32 - coderedart/egui\_overlay (github.com)](https://github.com/coderedart/egui_overlay/issues/32)
- [winit::changelog::v0\_30 - Rust (docs.rs)](https://docs.rs/winit/latest/winit/changelog/v0_30/index.html)

## Reference - AI Agents

- [SysCV/sam-hq: Segment Anything in High Quality \[NeurIPS 2023\] (github.com)](https://github.com/SysCV/sam-hq)
- [OpenAdaptAI/OpenAdapt: AI-First Process Automation with Large (\[Language (LLMs) / Action (LAMs) / Multimodal (LMMs)\] / Visual Language (VLMs)) Models (github.com)](https://github.com/OpenAdaptAI/OpenAdapt)
  - [OpenAdapt.AI](https://openadapt.ai/)
  - [(12) Richard Abrich on X: "Check out the latest from @OpenAdaptAI. Now you can quickly and easily automate repetitive tasks in desktop apps for free! Compatible with @OpenAI, @Anthropic, @GoogleAI, and more. Go to https://t.co/7WzHcDRrRH to get started. https://t.co/KkC24CxJFK" / X (twitter.com)](https://twitter.com/abrichr/status/1784307190062342237)
- [OpenInterpreter/open-interpreter: A natural language interface for computers (github.com)](https://github.com/OpenInterpreter/open-interpreter)
  - [Open Interpreter - YouTube](https://www.youtube.com/@OpenInterpreter)
  - [George Kedenburg III on X: "ai pin 🤝 open interpreter https://t.co/cUU2zGoCfd" / X (twitter.com)](https://twitter.com/GK3/status/1773159515258495257)
- [Techfrens AI Agents Rubrik - Google Drive](https://docs.google.com/spreadsheets/u/0/d/19uE7EzGv-uqH7JyjG0FpC4mD21HPTeP6SWXpVhvNcxI/htmlview#)
- [jordan singer on X: "✨ talk to your computer remotely from your phone i call it Teleport https://t.co/EvktAB1Lrz" / X (twitter.com)](https://twitter.com/jsngr/status/1774110742070882478)
- [princeton-nlp/SWE-agent: SWE-agent takes a GitHub issue and tries to automatically fix it, using GPT-4, or your LM of choice. It solves 12.29% of bugs in the SWE-bench evaluation set and takes just 1.5 minutes to run.](https://github.com/princeton-nlp/SWE-agent)
  - [John Yang @ ICLR 🇦🇹 on X: "SWE-agent is our new system for autonomously solving issues in GitHub repos. It gets similar accuracy to Devin on SWE-bench, takes 93 seconds on avg + it's open source! We designed a new agent-computer interface to make it easy for GPT-4 to edit+run code https://t.co/CTzMxDiouH https://t.co/VW9FuZGIUf" / X (twitter.com)](https://twitter.com/jyangballin/status/1775114444370051582)

## Articles

- [Why it is time to start thinking of games as databases | by Sander Mertens | Medium](https://ajmmertens.medium.com/why-it-is-time-to-start-thinking-of-games-as-databases-e7971da33ac3)

## Screen Capturing / Recording

- [Show HN: I made an open-source Loom alternative | Hacker News (ycombinator.com)](https://news.ycombinator.com/item?id=40338275)
  - [Capture/Desktop -- FFmpeg](https://trac.ffmpeg.org/wiki/Capture/Desktop)
  - [Cap --- Effortless, instant screen sharing. Open source and cross-platform.](https://cap.so/)