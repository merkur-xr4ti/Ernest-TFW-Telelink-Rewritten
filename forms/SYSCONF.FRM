VERSION 2.00
Begin Form frmSysConfig 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   Caption         =   "System Configuration Settings"
   ClientHeight    =   5910
   ClientLeft      =   2055
   ClientTop       =   3255
   ClientWidth     =   8895
   Height          =   6420
   Left            =   1995
   LinkTopic       =   "Form1"
   MaxButton       =   0   'False
   ScaleHeight     =   5910
   ScaleWidth      =   8895
   Top             =   2805
   Width           =   9015
   Begin Frame Frame3 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Modem Volume"
      ForeColor       =   &H00FF0000&
      Height          =   615
      Left            =   120
      TabIndex        =   58
      Top             =   4560
      Width           =   3135
      Begin OptionButton optModemVolume 
         BackColor       =   &H00C0C0C0&
         Caption         =   "High"
         Height          =   255
         Index           =   3
         Left            =   2280
         TabIndex        =   62
         Top             =   240
         Width           =   615
      End
      Begin OptionButton optModemVolume 
         BackColor       =   &H00C0C0C0&
         Caption         =   "Med"
         Height          =   255
         Index           =   2
         Left            =   1560
         TabIndex        =   61
         Top             =   240
         Width           =   615
      End
      Begin OptionButton optModemVolume 
         BackColor       =   &H00C0C0C0&
         Caption         =   "Low"
         Height          =   255
         Index           =   1
         Left            =   840
         TabIndex        =   60
         Top             =   240
         Width           =   615
      End
      Begin OptionButton optModemVolume 
         BackColor       =   &H00C0C0C0&
         Caption         =   "Off"
         Height          =   255
         Index           =   0
         Left            =   240
         TabIndex        =   59
         Top             =   240
         Width           =   495
      End
   End
   Begin Frame Frame2 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Printer Font Size"
      ForeColor       =   &H00FF0000&
      Height          =   615
      Left            =   6600
      TabIndex        =   56
      Top             =   4560
      Width           =   2175
      Begin MaskEdBox MaskedEdit1 
      End
   End
   Begin SSFrame Frame3D1 
      Begin SSCheck chkMinimum 
      End
      Begin SSCheck chkMonthlyTotals 
      End
      Begin SSCheck chkUnpolled 
      End
      Begin SSCheck chkFlags 
      End
   End
   Begin Frame Frame1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Country Font"
      ForeColor       =   &H00FF0000&
      Height          =   735
      Left            =   6600
      TabIndex        =   44
      Top             =   3720
      Width           =   2175
      Begin OptionButton IntON 
         BackColor       =   &H00C0C0C0&
         Caption         =   "ON"
         Height          =   255
         Left            =   1200
         TabIndex        =   45
         Top             =   360
         Value           =   -1  'True
         Width           =   735
      End
      Begin OptionButton IntOFF 
         BackColor       =   &H00C0C0C0&
         Caption         =   "OFF"
         Height          =   255
         Left            =   360
         TabIndex        =   46
         Top             =   360
         Width           =   735
      End
   End
   Begin Frame DCW 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Disable Call Waiting"
      ForeColor       =   &H00FF0000&
      Height          =   1695
      Left            =   6600
      TabIndex        =   39
      Top             =   960
      Width           =   2175
      Begin MaskEdBox DCWME 
      End
      Begin OptionButton DCWNo 
         BackColor       =   &H00C0C0C0&
         Caption         =   "NO"
         Height          =   252
         Left            =   1200
         TabIndex        =   41
         Top             =   360
         Value           =   -1  'True
         Width           =   492
      End
      Begin OptionButton DCWYes 
         BackColor       =   &H00C0C0C0&
         Caption         =   "YES"
         Height          =   252
         Left            =   480
         TabIndex        =   40
         Top             =   360
         Width           =   612
      End
      Begin Label DCWInfo 
         BackColor       =   &H00C0C0C0&
         BackStyle       =   0   'Transparent
         Caption         =   "Enter your 2 digit code."
         ForeColor       =   &H00800000&
         Height          =   255
         Left            =   120
         TabIndex        =   42
         Top             =   1200
         Width           =   2055
      End
   End
   Begin Frame fraModem 
      BackColor       =   &H00C0C0C0&
      Caption         =   " D1 Setup"
      ForeColor       =   &H00FF0000&
      Height          =   1695
      Index           =   3
      Left            =   3360
      TabIndex        =   38
      Top             =   3480
      Width           =   3135
      Begin TextBox txtInitString 
         FontBold        =   0   'False
         FontItalic      =   0   'False
         FontName        =   "MS Sans Serif"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   285
         Index           =   2
         Left            =   120
         TabIndex        =   14
         Top             =   600
         Width           =   2895
      End
      Begin TextBox txtInitAddon 
         FontBold        =   0   'False
         FontItalic      =   0   'False
         FontName        =   "MS Sans Serif"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   285
         Index           =   2
         Left            =   120
         TabIndex        =   15
         Top             =   1200
         Width           =   2895
      End
      Begin Label lblModemInit 
         BackStyle       =   0   'Transparent
         Caption         =   "Additional commands to send:"
         Height          =   255
         Index           =   5
         Left            =   120
         TabIndex        =   36
         Top             =   960
         Width           =   2895
      End
      Begin Label lblModemInit 
         BackStyle       =   0   'Transparent
         Caption         =   "Standard commands sent:"
         Height          =   255
         Index           =   4
         Left            =   120
         TabIndex        =   37
         Top             =   360
         Width           =   2895
      End
   End
   Begin Frame fraModem 
      BackColor       =   &H00C0C0C0&
      Caption         =   " D3 Setup"
      ForeColor       =   &H00FF0000&
      Height          =   1695
      Index           =   2
      Left            =   3360
      TabIndex        =   33
      Top             =   1680
      Width           =   3135
      Begin TextBox txtInitString 
         FontBold        =   0   'False
         FontItalic      =   0   'False
         FontName        =   "MS Sans Serif"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   285
         Index           =   1
         Left            =   120
         TabIndex        =   12
         Top             =   600
         Width           =   2895
      End
      Begin TextBox txtInitAddon 
         FontBold        =   0   'False
         FontItalic      =   0   'False
         FontName        =   "MS Sans Serif"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   285
         Index           =   1
         Left            =   120
         TabIndex        =   13
         Top             =   1200
         Width           =   2895
      End
      Begin Label lblModemInit 
         BackStyle       =   0   'Transparent
         Caption         =   "Additional commands to send:"
         Height          =   255
         Index           =   3
         Left            =   120
         TabIndex        =   35
         Top             =   960
         Width           =   2895
      End
      Begin Label lblModemInit 
         BackStyle       =   0   'Transparent
         Caption         =   "Standard commands sent:"
         Height          =   255
         Index           =   2
         Left            =   120
         TabIndex        =   34
         Top             =   360
         Width           =   2895
      End
   End
   Begin Frame fraModem 
      BackColor       =   &H00C0C0C0&
      Caption         =   " Modem Initialization"
      ForeColor       =   &H00FF0000&
      Height          =   1575
      Index           =   1
      Left            =   3360
      TabIndex        =   30
      Top             =   0
      Width           =   3135
      Begin TextBox txtInitAddon 
         FontBold        =   0   'False
         FontItalic      =   0   'False
         FontName        =   "MS Sans Serif"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   285
         Index           =   0
         Left            =   120
         TabIndex        =   11
         Top             =   1200
         Width           =   2895
      End
      Begin TextBox txtInitString 
         FontBold        =   0   'False
         FontItalic      =   0   'False
         FontName        =   "MS Sans Serif"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   285
         Index           =   0
         Left            =   120
         TabIndex        =   10
         Top             =   600
         Width           =   2895
      End
      Begin Label lblModemInit 
         BackStyle       =   0   'Transparent
         Caption         =   "Standard commands sent:"
         Height          =   255
         Index           =   1
         Left            =   120
         TabIndex        =   32
         Top             =   360
         Width           =   2895
      End
      Begin Label lblModemInit 
         BackStyle       =   0   'Transparent
         Caption         =   "Additional commands to send:"
         Height          =   255
         Index           =   0
         Left            =   120
         TabIndex        =   31
         Top             =   960
         Width           =   2895
      End
   End
   Begin CommandButton cmdCancel 
      Cancel          =   -1  'True
      Caption         =   "&Cancel"
      Height          =   495
      Left            =   5640
      TabIndex        =   20
      Top             =   5280
      Width           =   1335
   End
   Begin CommandButton cmdOk 
      Caption         =   "&Ok"
      Default         =   -1  'True
      Height          =   495
      Left            =   2400
      TabIndex        =   19
      Top             =   5280
      Width           =   1335
   End
   Begin Frame fraDialing 
      BackColor       =   &H00C0C0C0&
      Caption         =   " Behind a PBX ?"
      ForeColor       =   &H00FF0000&
      Height          =   855
      Index           =   1
      Left            =   120
      TabIndex        =   27
      Top             =   2640
      Width           =   3135
      Begin OptionButton optPBX 
         BackColor       =   &H00C0C0C0&
         Caption         =   "NO"
         Height          =   255
         Index           =   0
         Left            =   120
         TabIndex        =   8
         Top             =   480
         Width           =   615
      End
      Begin OptionButton optPBX 
         BackColor       =   &H00C0C0C0&
         Caption         =   "YES"
         Height          =   255
         Index           =   1
         Left            =   120
         TabIndex        =   7
         Top             =   240
         Width           =   735
      End
      Begin MaskEdBox edtPBX 
      End
      Begin Label lblPBX 
         Alignment       =   1   'Right Justify
         BackStyle       =   0   'Transparent
         Caption         =   "PBX Prefix:"
         Height          =   255
         Left            =   840
         TabIndex        =   28
         Top             =   360
         Width           =   1335
      End
   End
   Begin Frame fraDialing 
      BackColor       =   &H00C0C0C0&
      Caption         =   " Dialing Timeouts (seconds)"
      ForeColor       =   &H00FF0000&
      Height          =   1095
      Index           =   0
      Left            =   120
      TabIndex        =   24
      Top             =   1440
      Width           =   3135
      Begin SpinButton spinDialTime 
      End
      Begin SpinButton spinDialTime 
      End
      Begin Label lblDialTime 
         Alignment       =   1   'Right Justify
         BorderStyle     =   1   'Fixed Single
         Caption         =   "0"
         Height          =   255
         Index           =   3
         Left            =   1680
         TabIndex        =   6
         Top             =   720
         Width           =   495
      End
      Begin Label lblDialTime 
         Alignment       =   1   'Right Justify
         BackStyle       =   0   'Transparent
         Caption         =   "Long Distance:"
         Height          =   255
         Index           =   2
         Left            =   240
         TabIndex        =   26
         Top             =   720
         Width           =   1335
      End
      Begin Label lblDialTime 
         Alignment       =   1   'Right Justify
         BorderStyle     =   1   'Fixed Single
         Caption         =   "0"
         Height          =   255
         Index           =   1
         Left            =   1680
         TabIndex        =   5
         Top             =   360
         Width           =   495
      End
      Begin Label lblDialTime 
         Alignment       =   1   'Right Justify
         BackStyle       =   0   'Transparent
         Caption         =   "Local:"
         Height          =   255
         Index           =   0
         Left            =   240
         TabIndex        =   25
         Top             =   360
         Width           =   1335
      End
   End
   Begin Frame fraSystem 
      BackColor       =   &H00C0C0C0&
      Caption         =   " Enable Sounds "
      ForeColor       =   &H00FF0000&
      Height          =   855
      Index           =   0
      Left            =   6600
      TabIndex        =   23
      Top             =   0
      Width           =   2175
      Begin OptionButton optSound 
         BackColor       =   &H00C0C0C0&
         Caption         =   "YES"
         Height          =   252
         Index           =   1
         Left            =   480
         TabIndex        =   17
         Top             =   360
         Width           =   732
      End
      Begin OptionButton optSound 
         BackColor       =   &H00C0C0C0&
         Caption         =   "NO"
         Height          =   255
         Index           =   0
         Left            =   1200
         TabIndex        =   16
         Top             =   360
         Width           =   615
      End
   End
   Begin Frame fraSystem 
      BackColor       =   &H00C0C0C0&
      Caption         =   " Country "
      ForeColor       =   &H00FF0000&
      Height          =   855
      Index           =   1
      Left            =   6600
      TabIndex        =   22
      Top             =   2760
      Width           =   2175
      Begin ComboBox cboCountry 
         Height          =   315
         Left            =   240
         Style           =   2   'Dropdown List
         TabIndex        =   18
         Top             =   360
         Width           =   1695
      End
   End
   Begin Frame fraModem 
      BackColor       =   &H00C0C0C0&
      Caption         =   " Modem Port "
      ForeColor       =   &H00FF0000&
      Height          =   1335
      Index           =   0
      Left            =   120
      TabIndex        =   21
      Top             =   0
      Width           =   3135
      Begin OptionButton optSerialPort 
         BackColor       =   &H00C0C0C0&
         Caption         =   "7"
         ForeColor       =   &H00000000&
         Height          =   255
         Index           =   7
         Left            =   240
         TabIndex        =   50
         Top             =   960
         Width           =   375
      End
      Begin OptionButton optSerialPort 
         BackColor       =   &H00C0C0C0&
         Caption         =   "8"
         ForeColor       =   &H00000000&
         Height          =   255
         Index           =   8
         Left            =   720
         TabIndex        =   49
         Top             =   960
         Width           =   375
      End
      Begin OptionButton optSerialPort 
         BackColor       =   &H00C0C0C0&
         Caption         =   "6"
         ForeColor       =   &H00000000&
         Height          =   255
         Index           =   6
         Left            =   720
         TabIndex        =   48
         Top             =   720
         Width           =   375
      End
      Begin OptionButton optSerialPort 
         BackColor       =   &H00C0C0C0&
         Caption         =   "5"
         ForeColor       =   &H00000000&
         Height          =   255
         Index           =   5
         Left            =   240
         TabIndex        =   47
         Top             =   720
         Width           =   375
      End
      Begin SpinButton spinDelay 
      End
      Begin OptionButton optSerialPort 
         BackColor       =   &H00C0C0C0&
         Caption         =   "1"
         ForeColor       =   &H00000000&
         Height          =   255
         Index           =   1
         Left            =   240
         TabIndex        =   0
         Top             =   240
         Value           =   -1  'True
         Width           =   375
      End
      Begin OptionButton optSerialPort 
         BackColor       =   &H00C0C0C0&
         Caption         =   "2"
         ForeColor       =   &H00000000&
         Height          =   255
         Index           =   2
         Left            =   720
         TabIndex        =   1
         Top             =   240
         Width           =   375
      End
      Begin OptionButton optSerialPort 
         BackColor       =   &H00C0C0C0&
         Caption         =   "3"
         ForeColor       =   &H00000000&
         Height          =   255
         Index           =   3
         Left            =   240
         TabIndex        =   2
         Top             =   480
         Width           =   375
      End
      Begin OptionButton optSerialPort 
         BackColor       =   &H00C0C0C0&
         Caption         =   "4"
         ForeColor       =   &H00000000&
         Height          =   255
         Index           =   4
         Left            =   720
         TabIndex        =   3
         Top             =   480
         Width           =   375
      End
      Begin Label lblDelayVal 
         Alignment       =   1   'Right Justify
         BorderStyle     =   1   'Fixed Single
         Caption         =   "0"
         Height          =   255
         Left            =   1680
         TabIndex        =   4
         Top             =   840
         Width           =   495
      End
      Begin Label lblDelay 
         Alignment       =   2   'Center
         BackStyle       =   0   'Transparent
         Caption         =   "Inter-command delay (ms)"
         ForeColor       =   &H00008000&
         Height          =   495
         Left            =   1560
         TabIndex        =   29
         Top             =   240
         Width           =   1335
      End
   End
End

