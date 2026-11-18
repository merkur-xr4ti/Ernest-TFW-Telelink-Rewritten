VERSION 2.00
Begin Form frmBatchSchedule 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   Caption         =   "Batch Schedule Setup"
   ClientHeight    =   4680
   ClientLeft      =   510
   ClientTop       =   2025
   ClientWidth     =   6015
   Height          =   5190
   Left            =   450
   LinkTopic       =   "Form1"
   MaxButton       =   0   'False
   ScaleHeight     =   4680
   ScaleWidth      =   6015
   Top             =   1575
   Width           =   6135
   Begin CheckBox chkStartup 
      BackColor       =   &H00C0C0C0&
      Caption         =   " Turn AutoCall on when Telelink starts"
      Height          =   255
      Left            =   0
      TabIndex        =   33
      Top             =   0
      Width           =   3735
   End
   Begin TextBox txtCallAttempts 
      Alignment       =   2   'Center
      Height          =   285
      HideSelection   =   0   'False
      Left            =   2400
      MaxLength       =   1
      TabIndex        =   32
      Text            =   "1"
      Top             =   3720
      Width           =   255
   End
   Begin CommandButton cmdQuitList 
      Caption         =   "&Quit"
      Height          =   255
      Left            =   4560
      TabIndex        =   26
      Top             =   3720
      Width           =   735
   End
   Begin CommandButton cmdSaveList 
      Caption         =   "&Save"
      Height          =   255
      Left            =   3480
      TabIndex        =   24
      Top             =   3720
      Width           =   735
   End
   Begin FileListBox fileBatchFiles 
      Height          =   2565
      Left            =   3360
      MultiSelect     =   2
      Pattern         =   "*.bxw"
      TabIndex        =   23
      Top             =   1080
      Width           =   1455
   End
   Begin CommandButton cmdCancel 
      Cancel          =   -1  'True
      Caption         =   "&Cancel"
      Height          =   375
      Left            =   3120
      TabIndex        =   22
      Top             =   4200
      Width           =   855
   End
   Begin CommandButton cmdOk 
      Caption         =   "&Ok"
      Default         =   -1  'True
      Height          =   375
      Left            =   1920
      TabIndex        =   21
      Top             =   4200
      Width           =   855
   End
   Begin SSFrame frmSchedule 
      Begin SSCheck chkDayOfWeek 
      End
      Begin SSCheck chkDayOfWeek 
      End
      Begin SSCheck chkDayOfWeek 
      End
      Begin SSCheck chkDayOfWeek 
      End
      Begin SSCheck chkDayOfWeek 
      End
      Begin SSCheck chkDayOfWeek 
      End
      Begin SSCheck chkDayOfWeek 
      End
      Begin MaskEdBox edtStartTime 
      End
      Begin MaskEdBox edtStartTime 
      End
      Begin MaskEdBox edtStartTime 
      End
      Begin MaskEdBox edtStartTime 
      End
      Begin MaskEdBox edtStartTime 
      End
      Begin MaskEdBox edtStartTime 
      End
      Begin MaskEdBox edtStartTime 
      End
   End
   Begin Label lblCallAttempts 
      AutoSize        =   -1  'True
      BackStyle       =   0   'Transparent
      Caption         =   "Call Attempts per Phone:"
      Height          =   195
      Left            =   240
      TabIndex        =   31
      Top             =   3720
      Width           =   2115
   End
   Begin Label Label4 
      Alignment       =   2   'Center
      AutoSize        =   -1  'True
      BackStyle       =   0   'Transparent
      Caption         =   "Military Time (HHMM)"
      FontBold        =   0   'False
      FontItalic      =   0   'False
      FontName        =   "MS Sans Serif"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      ForeColor       =   &H00FF0000&
      Height          =   195
      Left            =   1575
      TabIndex        =   30
      Top             =   480
      Width           =   1545
   End
   Begin Label Label3 
      AutoSize        =   -1  'True
      BackStyle       =   0   'Transparent
      Caption         =   "Batch Files to Run"
      FontBold        =   -1  'True
      FontItalic      =   0   'False
      FontName        =   "MS Sans Serif"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   -1  'True
      Height          =   195
      Left            =   3000
      TabIndex        =   29
      Top             =   720
      Width           =   1590
   End
   Begin Label Label2 
      AutoSize        =   -1  'True
      BackStyle       =   0   'Transparent
      Caption         =   "Start Time"
      FontBold        =   -1  'True
      FontItalic      =   0   'False
      FontName        =   "MS Sans Serif"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   -1  'True
      Height          =   195
      Left            =   1800
      TabIndex        =   28
      Top             =   720
      Width           =   885
   End
   Begin Label Label1 
      AutoSize        =   -1  'True
      BackStyle       =   0   'Transparent
      Caption         =   "Day of Week"
      FontBold        =   -1  'True
      FontItalic      =   0   'False
      FontName        =   "MS Sans Serif"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   -1  'True
      Height          =   195
      Left            =   240
      TabIndex        =   27
      Top             =   720
      Width           =   1125
   End
   Begin Label lblFileList 
      BorderStyle     =   1   'Fixed Single
      Height          =   255
      Index           =   6
      Left            =   3000
      TabIndex        =   20
      Top             =   3240
      Width           =   2775
   End
   Begin Label lblFileList 
      BorderStyle     =   1   'Fixed Single
      Height          =   255
      Index           =   5
      Left            =   3000
      TabIndex        =   17
      Top             =   2880
      Width           =   2775
   End
   Begin Label lblFileList 
      BorderStyle     =   1   'Fixed Single
      Height          =   255
      Index           =   4
      Left            =   3000
      TabIndex        =   14
      Top             =   2520
      Width           =   2775
   End
   Begin Label lblFileList 
      BorderStyle     =   1   'Fixed Single
      Height          =   255
      Index           =   3
      Left            =   3000
      TabIndex        =   11
      Top             =   2160
      Width           =   2775
   End
   Begin Label lblFileList 
      BorderStyle     =   1   'Fixed Single
      Height          =   255
      Index           =   2
      Left            =   3000
      TabIndex        =   8
      Top             =   1800
      Width           =   2775
   End
   Begin Label lblFileList 
      BorderStyle     =   1   'Fixed Single
      Height          =   255
      Index           =   1
      Left            =   3000
      TabIndex        =   5
      Top             =   1440
      Width           =   2775
   End
   Begin Label lblFileList 
      BorderStyle     =   1   'Fixed Single
      Height          =   255
      Index           =   0
      Left            =   3000
      TabIndex        =   2
      Top             =   1080
      Width           =   2775
   End
End

