VERSION 2.00
Begin Form frmProgrammableFeatures 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   Caption         =   "Programmable Features"
   ClientHeight    =   4500
   ClientLeft      =   270
   ClientTop       =   1980
   ClientWidth     =   9135
   ControlBox      =   0   'False
   Height          =   5010
   Left            =   210
   LinkTopic       =   "Form5"
   MaxButton       =   0   'False
   ScaleHeight     =   4500
   ScaleWidth      =   9135
   Top             =   1530
   Width           =   9255
   Begin FileListBox filFEAS 
      Height          =   420
      Left            =   480
      Pattern         =   "v5##-##?.fea"
      TabIndex        =   45
      Top             =   4440
      Visible         =   0   'False
      Width           =   1215
   End
   Begin FileListBox File1 
      Height          =   225
      Left            =   8160
      TabIndex        =   44
      Top             =   3720
      Visible         =   0   'False
      Width           =   615
   End
   Begin CommandButton cmdClear 
      Caption         =   "C&lear"
      Height          =   375
      Left            =   4920
      TabIndex        =   43
      Top             =   3960
      Width           =   1095
   End
   Begin CommandButton cmdUntagAll 
      Caption         =   "Untag All"
      Height          =   375
      Left            =   3480
      TabIndex        =   41
      Top             =   3960
      Width           =   1335
   End
   Begin CommandButton cmdTagAll 
      Caption         =   "Tag All"
      Height          =   375
      Left            =   2040
      TabIndex        =   40
      Top             =   3960
      Width           =   1335
   End
   Begin CommandButton cmdToggleTagAll 
      Caption         =   "Toggle &All Tags"
      Height          =   375
      Left            =   120
      TabIndex        =   38
      Top             =   3960
      Width           =   1815
   End
   Begin SSFrame Frame3D1 
      Begin Label Label2 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   11
         Left            =   120
         TabIndex        =   37
         Top             =   720
         Width           =   2055
      End
      Begin Label Label1 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   255
         Index           =   11
         Left            =   120
         TabIndex        =   36
         Top             =   360
         Width           =   2055
      End
   End
   Begin SSFrame Frame3D1 
      Begin Label Label2 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   10
         Left            =   120
         TabIndex        =   34
         Top             =   720
         Width           =   2055
      End
      Begin Label Label1 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   10
         Left            =   120
         TabIndex        =   33
         Top             =   360
         Width           =   2055
      End
   End
   Begin SSFrame Frame3D1 
      Begin Label Label2 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   9
         Left            =   120
         TabIndex        =   31
         Top             =   720
         Width           =   2055
      End
      Begin Label Label1 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   9
         Left            =   120
         TabIndex        =   30
         Top             =   360
         Width           =   2055
      End
   End
   Begin SSFrame Frame3D1 
      Begin Label Label2 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   8
         Left            =   120
         TabIndex        =   28
         Top             =   720
         Width           =   2055
      End
      Begin Label Label1 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   8
         Left            =   120
         TabIndex        =   27
         Top             =   360
         Width           =   2055
      End
   End
   Begin SSFrame Frame3D1 
      Begin Label Label2 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   7
         Left            =   120
         TabIndex        =   25
         Top             =   720
         Width           =   2055
      End
      Begin Label Label1 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   7
         Left            =   120
         TabIndex        =   24
         Top             =   360
         Width           =   2055
      End
   End
   Begin SSFrame Frame3D1 
      Begin Label Label2 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   6
         Left            =   120
         TabIndex        =   22
         Top             =   720
         Width           =   2055
      End
      Begin Label Label1 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   6
         Left            =   120
         TabIndex        =   21
         Top             =   360
         Width           =   2055
      End
   End
   Begin SSFrame Frame3D1 
      Begin Label Label2 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   5
         Left            =   120
         TabIndex        =   19
         Top             =   720
         Width           =   2055
      End
      Begin Label Label1 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   5
         Left            =   120
         TabIndex        =   18
         Top             =   360
         Width           =   2055
      End
   End
   Begin SSFrame Frame3D1 
      Begin Label Label2 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   4
         Left            =   120
         TabIndex        =   16
         Top             =   720
         Width           =   2055
      End
      Begin Label Label1 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   4
         Left            =   120
         TabIndex        =   15
         Top             =   360
         Width           =   2055
      End
   End
   Begin SSFrame Frame3D1 
      Begin Label Label2 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   3
         Left            =   120
         TabIndex        =   13
         Top             =   720
         Width           =   2055
      End
      Begin Label Label1 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   3
         Left            =   120
         TabIndex        =   12
         Top             =   360
         Width           =   2055
      End
   End
   Begin SSFrame Frame3D1 
      Begin Label Label2 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   2
         Left            =   120
         TabIndex        =   10
         Top             =   720
         Width           =   2055
      End
      Begin Label Label1 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   2
         Left            =   120
         TabIndex        =   9
         Top             =   360
         Width           =   2055
      End
   End
   Begin SSFrame Frame3D1 
      Begin Label Label2 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   1
         Left            =   120
         TabIndex        =   7
         Top             =   720
         Width           =   2055
      End
      Begin Label Label1 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   1
         Left            =   120
         TabIndex        =   6
         Top             =   360
         Width           =   2055
      End
   End
   Begin CommandButton cmdNext 
      Caption         =   "&Next"
      Height          =   375
      Left            =   7800
      TabIndex        =   0
      Top             =   3960
      Width           =   1215
   End
   Begin CommandButton cmdPrevious 
      Caption         =   "&Previous"
      Height          =   375
      Left            =   6480
      TabIndex        =   1
      Top             =   3960
      Width           =   1215
   End
   Begin SSFrame Frame3D1 
      Begin Label Label1 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   0
         Left            =   120
         TabIndex        =   3
         Top             =   360
         Width           =   2055
      End
      Begin Label Label2 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Index           =   0
         Left            =   120
         TabIndex        =   4
         Top             =   720
         Width           =   2055
      End
   End
   Begin Label lblChange 
      BackStyle       =   0   'Transparent
      Caption         =   "Don't forget to save your changes!"
      ForeColor       =   &H000000FF&
      Height          =   255
      Left            =   4800
      TabIndex        =   42
      Tag             =   "No"
      Top             =   3600
      Visible         =   0   'False
      Width           =   3855
   End
   Begin Label Label3 
      BackColor       =   &H00C0C0C0&
      BackStyle       =   0   'Transparent
      Height          =   255
      Left            =   420
      TabIndex        =   39
      Top             =   3720
      Width           =   4335
   End
End

