# TFW — TeleLink for Windows, rewritten in Rust

A form-by-form rewrite of a 1990s Visual Basic 3 payphone management program (TFW.EXE,
"TeleLink for Windows") for Ernest D1/D3 payphones. The original is a 16-bit program that no
longer runs on 64-bit Windows. This rewrite follows the decompiled VB3 source as closely as
possible: the same windows and control positions, the same captions, the same files and the
same modem protocol.

**This is a Rust rewrite of decompiled code, completed with the use of AI.** Parts the
decompiler lost are reconstructed and marked *ESTIMATED* in the code (see "Known gaps").

## What it does

* **Main window**: date and time, AutoCall and AutoAnswer schedules, Accept Incoming, the D3/D1
  modem setting, the connection state and the "Check Flags!!" alert.
* **Direct** (talks to one phone over a modem): AOS scripts, collection status, CSN/UPT tables,
  programmable features (with tagging and tagged download), rate table, SMDR, speed dial, time
  and date, scrolling message and firmware.
* **Batch**: phone lists, batch files, batch execution with retries and a batch log.
* **Settings, Reports, Flags**: AutoCall/AutoAnswer schedules, system configuration, the report
  generator with graphs, call detail records, and call flags.

## Download

`bin/tfw.exe` is a ready-built 64-bit Windows program: one file, nothing to install. Put it
anywhere and run it.

## Building

Needs Rust (stable, 2024 edition).

```
cargo build --release
```

The program is `target/release/tfw.exe`.

## Running

Like the original, it keeps its data in `C:\TFW`: settings in `C:\TFW\TFW.INI`, window captions
in `C:\TFW\TEXT\USA.INI` (CHINA/TAIWAN.INI when *International* is on), features files in
`C:\TFW\FEA`, and the BXW, PLW, CSN, SDN, AOS, RTF, SMD, CFW... folders. Set `TFW_ROOT` to use
another folder. Without a caption table, neutral built-in captions are used. Files from the
original installation aren't part of this repository.

### Phone line

*Settings > System Configuration > Phone Line* picks how calls go out:

* **Modem on port**: a Hayes modem on any COM port (the original's COM1-8 buttons still work),
  at 1200 or 300 baud, 7 data bits, even parity, 1 stop bit, as the original used it. A virtual
  modem on a com0com port pair works too.
* **SIP modem (built in)**: no modem needed. Calls go out through a SIP account (server, port,
  user, password) as G.711 u-law audio carrying real modem tones: Bell 212A at 1200 bit/s for
  D3 phones, Bell 103 at 300 bit/s for D1. It takes the same AT commands a Hayes modem does, so
  the rest of the program can't tell the difference. Outgoing calls only; the provider must
  accept calls without a registration (most do, with a digest challenge on each call).

These are kept in TFW.INI:

```
[Modem]
Line=COM            ; or SIP
SerialPort=2

[SipModem]
Server=sip.example.com
Port=5060
User=myaccount
Password=...        ; stored as plain text, like the rest of TFW.INI
```

Command-line options:

| option | what it does |
|---|---|
| `--screenshot <file.png>` | start, draw about 120 frames, save the main window as a PNG, exit (no modem) |
| `--selftest` | open every one of the 41 windows without showing them and click their controls |
| `--calltest <number> <id> [D1] [write]` | call a phone and run every Direct read; with `write`, write the values back |

For checking layouts, `TFW_PREVIEW_FORM=<form name>` with `--screenshot` draws that form in the
main window, and `TFW_PREVIEW_FEA=<file.fea>` opens a features file in Programmable Features.

## How the code is laid out

| file | contents |
|---|---|
| `forms/*.FRM` | the layout blocks of the original forms (twips), read at start-up |
| `forms/vbx.txt` | positions of the VBX controls (3-D frames, masked edits, grids...) the decompiler lost (*ESTIMATED*) |
| `src/vb.rs` | a small VB3 form runtime: parses the .FRM blocks, draws the controls in the Windows 3.1 look, turns clicks and edits into events, message boxes |
| `src/app.rs` | the project: loaded forms, shared globals (Globals.bas), one window per visible form, timers |
| `src/forms/` | the form code, one module per group of forms (`main_form`, `connect`, `direct`, `lists`, `batch`, `sysconf`, `logs`, `reports`) |
| `src/modem.rs` | the modem line (the MSComm control: a COM port or the SIP modem) on its own thread, and every transfer |
| `src/sip/` | the built-in SIP modem: AT commands and the SIP call (`mod.rs`), SIP messages and digest sign-in (`msg.rs`), the Bell 212A/103 software modem (`softmodem.rs`) |
| `src/proto.rs` | the payphone protocol: log-in, `!2` reads, `!1` writes, memory layouts for D3 and D1 phones |
| `src/files.rs`, `src/fea.rs` | the data files (.SDN, .CSN, .AOS, .BXW, .PLW, .SMD, .CFW, .FEA) |
| `src/text.rs`, `src/ini.rs` | GetPrivateProfileString-style INI access (TFW.INI, the caption table) |
| `src/calltest.rs` | the `--calltest` call test |

Names in the code follow the original: forms and controls keep their VB names
(`frmSpeedDial`, `MaskedEdit1(3)`, `Frame3D1(8)`), and comments name the decompiled routine a
piece of code comes from (`sub0F7D`, `fn0589`).

## Known gaps

* The decompiler lost the properties of the VBX controls and the menus; their positions and
  menu order are reconstructed (*ESTIMATED*).
* The record layouts of .PLW phone lists, .CFW collection logs and .FLG flag files aren't in the
  decompiled code; the rewrite uses its own fixed layouts (*ESTIMATED*), so files saved by the
  original may not open.
* What a phone sends when it calls in (AutoAnswer) isn't known; incoming calls are detected but
  not decoded.

## Tests

```
cargo test
```

Unit tests cover the masks, the form parser, INI handling, the features file format and the
protocol (records from recorded calls and a pretend phone).
