VERSION 2.00
Begin Form firmware 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   Caption         =   "Firmware"
   ClientHeight    =   4095
   ClientLeft      =   900
   ClientTop       =   2130
   ClientWidth     =   7440
   ControlBox      =   0   'False
   Height          =   4605
   Left            =   840
   LinkTopic       =   "Form1"
   MaxButton       =   0   'False
   ScaleHeight     =   4095
   ScaleWidth      =   7440
   Top             =   1680
   Width           =   7560
   Begin Frame control24 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Current Firmware Data"
      Height          =   2775
      Left            =   120
      TabIndex        =   0
      Top             =   360
      Width           =   3495
      Begin Label control25 
         Alignment       =   2   'Center
         BackColor       =   &H00C0C0C0&
         Caption         =   "Active Next Power-up11"
         ForeColor       =   &H00FF0000&
         Height          =   615
         Left            =   2160
         TabIndex        =   14
         Top             =   1920
         Width           =   975
      End
      Begin Label control8 
         Alignment       =   2   'Center
         BackColor       =   &H00C0C0C0&
         Caption         =   "Active Next Power-up1"
         ForeColor       =   &H00FF0000&
         Height          =   615
         Left            =   2160
         TabIndex        =   13
         Top             =   600
         Width           =   855
      End
      Begin Label control21 
         Alignment       =   2   'Center
         BackColor       =   &H00C0C0C0&
         Caption         =   "Active10"
         ForeColor       =   &H000000FF&
         Height          =   375
         Left            =   120
         TabIndex        =   1
         Top             =   2040
         Width           =   855
      End
      Begin Label control20 
         Alignment       =   2   'Center
         BackColor       =   &H00C0C0C0&
         Caption         =   "Active9"
         ForeColor       =   &H000000FF&
         Height          =   375
         Left            =   120
         TabIndex        =   2
         Top             =   720
         Width           =   855
      End
      Begin Label control10 
         Alignment       =   2   'Center
         Caption         =   "Label3"
         Height          =   255
         Left            =   1080
         TabIndex        =   5
         Top             =   2040
         Width           =   855
      End
      Begin Label control14 
         Alignment       =   2   'Center
         BackColor       =   &H00C0C0C0&
         Caption         =   "Secondary"
         Height          =   255
         Left            =   1040
         TabIndex        =   6
         Top             =   1680
         Width           =   975
      End
      Begin Label control9 
         Alignment       =   2   'Center
         Caption         =   "Label2"
         Height          =   255
         Left            =   1080
         TabIndex        =   11
         Top             =   720
         Width           =   855
      End
      Begin Label control13 
         Alignment       =   2   'Center
         BackColor       =   &H00C0C0C0&
         Caption         =   "Primary"
         Height          =   255
         Left            =   1080
         TabIndex        =   12
         Top             =   360
         Width           =   855
      End
   End
   Begin Gauge control18 
   End
   Begin OptionButton control16 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Run Secondary After Download"
      Height          =   255
      Left            =   4200
      TabIndex        =   8
      Top             =   2100
      Width           =   2955
   End
   Begin OptionButton control15 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Run Primary After Download"
      Height          =   255
      Left            =   4200
      TabIndex        =   7
      Top             =   1800
      Value           =   -1  'True
      Width           =   2955
   End
   Begin Label control19 
      Alignment       =   2   'Center
      BackColor       =   &H00C0C0C0&
      Caption         =   "Downloading"
      Height          =   255
      Left            =   1440
      TabIndex        =   10
      Top             =   3720
      Width           =   4095
   End
   Begin Label control12 
      Alignment       =   2   'Center
      Caption         =   "Secondary Ready to Download"
      Height          =   255
      Left            =   4920
      TabIndex        =   4
      Top             =   1200
      Width           =   1095
   End
   Begin Label control11 
      Alignment       =   2   'Center
      BackColor       =   &H00C0C0C0&
      Caption         =   "Loaded For Download To Secondary "
      Height          =   435
      Left            =   4320
      TabIndex        =   3
      Top             =   540
      Width           =   2355
   End
End
