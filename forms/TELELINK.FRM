VERSION 2.00
Begin Form Telelink 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   Caption         =   "t"
   ClientHeight    =   4860
   ClientLeft      =   135
   ClientTop       =   2565
   ClientWidth     =   8850
   ControlBox      =   0   'False
   FontBold        =   0   'False
   FontItalic      =   0   'False
   FontName        =   "MS Serif"
   FontSize        =   6.75
   FontStrikethru  =   0   'False
   FontUnderline   =   0   'False
   Height          =   5370
   Icon            =   TELELINK.FRX:0000
   Left            =   75
   LinkTopic       =   "Form1"
   MaxButton       =   0   'False
   ScaleHeight     =   4860
   ScaleWidth      =   8850
   Tag             =   "Main"
   Top             =   2115
   Width           =   8970
   Begin ListBox lstGetIDFiles 
      Height          =   225
      Left            =   120
      TabIndex        =   31
      Top             =   3480
      Visible         =   0   'False
      Width           =   1215
   End
   Begin ListBox List1 
      Height          =   225
      Left            =   120
      Sorted          =   -1  'True
      TabIndex        =   30
      Top             =   3120
      Visible         =   0   'False
      Width           =   1215
   End
   Begin FileListBox filGetIDFiles 
      Height          =   225
      Left            =   120
      TabIndex        =   29
      Top             =   2760
      Visible         =   0   'False
      Width           =   1215
   End
   Begin FileListBox filTFWCheck 
      Height          =   420
      Left            =   120
      TabIndex        =   28
      Top             =   2280
      Visible         =   0   'False
      Width           =   1215
   End
   Begin MSComm Modem 
   End
   Begin SSFrame Frame3D1 
      Begin Label lblAnswerSched 
         Alignment       =   2   'Center
         BackStyle       =   0   'Transparent
         Caption         =   "."
         FontBold        =   0   'False
         FontItalic      =   0   'False
         FontName        =   "MS Sans Serif"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   255
         Index           =   1
         Left            =   120
         TabIndex        =   10
         Top             =   480
         Width           =   2775
      End
      Begin Label lblAnswerSched 
         Alignment       =   2   'Center
         BackStyle       =   0   'Transparent
         Caption         =   "."
         FontBold        =   0   'False
         FontItalic      =   0   'False
         FontName        =   "MS Sans Serif"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   255
         Index           =   0
         Left            =   120
         TabIndex        =   27
         Top             =   240
         Width           =   2775
      End
   End
   Begin SSFrame Frame3D1 
      Begin SSCommand cmdAutoAnswer 
      End
      Begin Label lblAutoAnswer 
         Alignment       =   2   'Center
         BackStyle       =   0   'Transparent
         Caption         =   "Auto Answer"
         ForeColor       =   &H00FF0000&
         Height          =   330
         Index           =   0
         Left            =   120
         TabIndex        =   25
         Top             =   240
         Width           =   1575
      End
   End
   Begin SSFrame Frame3D1 
      Begin Label lblAlert 
         Alignment       =   2   'Center
         BackStyle       =   0   'Transparent
         Caption         =   "Check Flags!!"
         ForeColor       =   &H000000FF&
         Height          =   255
         Index           =   1
         Left            =   120
         TabIndex        =   21
         Top             =   480
         Visible         =   0   'False
         Width           =   2775
      End
      Begin Label lblAlert 
         Alignment       =   2   'Center
         BackStyle       =   0   'Transparent
         Caption         =   "Check Flags!!"
         ForeColor       =   &H000000FF&
         Height          =   255
         Index           =   0
         Left            =   120
         TabIndex        =   22
         Top             =   240
         Visible         =   0   'False
         Width           =   2775
      End
   End
   Begin SSFrame Frame3D1 
      Begin SSCommand cmdHangUp 
      End
      Begin Label lblConnection 
         Alignment       =   2   'Center
         BackStyle       =   0   'Transparent
         Caption         =   "Connected to 000-000-0000"
         ForeColor       =   &H00FF0000&
         Height          =   495
         Left            =   120
         TabIndex        =   19
         Top             =   120
         Width           =   2655
      End
   End
   Begin SSFrame Frame3D1 
      Begin SSCommand cmdAcceptIncoming 
      End
      Begin Label lblIncoming 
         Alignment       =   2   'Center
         BackStyle       =   0   'Transparent
         Caption         =   "Accept Incoming"
         ForeColor       =   &H00FF0000&
         Height          =   300
         Left            =   120
         TabIndex        =   16
         Top             =   240
         Width           =   1935
      End
   End
   Begin SSFrame Frame3D1 
      Begin Label lblNextRun 
         Alignment       =   2   'Center
         BackStyle       =   0   'Transparent
         Caption         =   "."
         FontBold        =   0   'False
         FontItalic      =   0   'False
         FontName        =   "MS Sans Serif"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   255
         Index           =   0
         Left            =   120
         TabIndex        =   11
         Top             =   240
         Width           =   2775
      End
      Begin Label lblNextRun 
         Alignment       =   2   'Center
         BackStyle       =   0   'Transparent
         Caption         =   "."
         FontBold        =   0   'False
         FontItalic      =   0   'False
         FontName        =   "MS Sans Serif"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   255
         Index           =   1
         Left            =   120
         TabIndex        =   13
         Top             =   480
         Width           =   2775
      End
   End
   Begin SSFrame Frame3D1 
      Begin SSCommand cmdPhoneType 
      End
      Begin Label lblPhoneType 
         Alignment       =   2   'Center
         BackStyle       =   0   'Transparent
         Caption         =   "Modem is set for"
         ForeColor       =   &H00FF0000&
         Height          =   375
         Left            =   120
         TabIndex        =   8
         Top             =   240
         Width           =   1935
      End
   End
   Begin SSFrame Frame3D1 
      Begin SSCommand cmdAutoCall 
      End
      Begin Label lblAutoCall 
         Alignment       =   2   'Center
         BackStyle       =   0   'Transparent
         Caption         =   "Auto Call"
         ForeColor       =   &H00FF0000&
         Height          =   375
         Left            =   120
         TabIndex        =   6
         Top             =   240
         Width           =   1575
      End
   End
   Begin SSFrame Frame3D1 
      Begin Label lblToday 
         Alignment       =   2   'Center
         BackStyle       =   0   'Transparent
         Caption         =   "."
         Height          =   255
         Index           =   0
         Left            =   120
         TabIndex        =   1
         Top             =   120
         Width           =   2295
      End
      Begin Label lblToday 
         Alignment       =   2   'Center
         BackStyle       =   0   'Transparent
         Caption         =   "."
         Height          =   255
         Index           =   1
         Left            =   120
         TabIndex        =   2
         Top             =   360
         Width           =   2295
      End
      Begin Label lblToday 
         Alignment       =   2   'Center
         BackStyle       =   0   'Transparent
         Caption         =   "."
         Height          =   255
         Index           =   2
         Left            =   120
         TabIndex        =   3
         Top             =   600
         Width           =   2295
      End
   End
   Begin Timer tmrAutoCall 
      Enabled         =   0   'False
      Interval        =   1000
      Left            =   135
      Top             =   1200
   End
   Begin MSComm Comm1 
   End
   Begin CommonDialog CMDialog1 
   End
   Begin Image Image1 
      Height          =   615
      Left            =   5640
      Top             =   3120
      Width           =   255
   End
End

