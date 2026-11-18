VERSION 2.00
Begin Form frmBatchExec 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   ClientHeight    =   6150
   ClientLeft      =   3045
   ClientTop       =   1260
   ClientWidth     =   5655
   Height          =   6660
   Left            =   2985
   LinkTopic       =   "Form1"
   MaxButton       =   0   'False
   ScaleHeight     =   6150
   ScaleWidth      =   5655
   Tag             =   "frmBatchExec"
   Top             =   810
   Width           =   5775
   Begin Timer SendKeysTimer 
      Interval        =   1000
      Left            =   4560
      Top             =   2640
   End
   Begin Timer tmrBatch 
      Enabled         =   0   'False
      Interval        =   1000
      Left            =   5280
      Top             =   2640
   End
   Begin Frame Frame1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Start Time -----> (Click Ok to Start)"
      Height          =   860
      Left            =   480
      TabIndex        =   0
      Top             =   3600
      Visible         =   0   'False
      Width           =   3135
      Begin Label lblScheduledStartTime 
         Alignment       =   2   'Center
         BorderStyle     =   1   'Fixed Single
         Height          =   260
         Left            =   840
         TabIndex        =   7
         Top             =   360
         Width           =   1335
      End
   End
   Begin CommandButton cmdViewUnpolled 
      Caption         =   "List Phones"
      Height          =   495
      Left            =   4440
      TabIndex        =   17
      Top             =   5280
      Visible         =   0   'False
      Width           =   1095
   End
   Begin Gauge Gauge1 
   End
   Begin ListBox List1 
      Height          =   2175
      Left            =   120
      TabIndex        =   6
      Top             =   2880
      Width           =   4095
   End
   Begin CommandButton cmdSkip 
      Caption         =   "&Skip"
      Height          =   500
      Left            =   4440
      TabIndex        =   4
      Top             =   4560
      Width           =   1095
   End
   Begin CommandButton cmdCancel 
      Cancel          =   -1  'True
      Caption         =   "&Cancel"
      Height          =   500
      Left            =   4440
      TabIndex        =   2
      Top             =   3840
      Width           =   1095
   End
   Begin CommandButton cmdOk 
      Caption         =   "&OK"
      Default         =   -1  'True
      Height          =   500
      Left            =   4440
      TabIndex        =   1
      Top             =   3120
      Width           =   1095
   End
   Begin Frame frmOptions 
      BackColor       =   &H00C0C0C0&
      Height          =   2415
      Left            =   0
      TabIndex        =   12
      Top             =   0
      Width           =   5655
      Begin ListBox lstBatchFileList 
         Height          =   615
         Left            =   720
         TabIndex        =   19
         Top             =   1200
         Width           =   2655
      End
      Begin OptionButton optRunLater 
         BackColor       =   &H00C0C0C0&
         Caption         =   "Run Later"
         Height          =   255
         Left            =   1560
         TabIndex        =   3
         Top             =   2040
         Width           =   1095
      End
      Begin TextBox txtBatchFileName 
         BackColor       =   &H00FFFFFF&
         Height          =   300
         Left            =   720
         TabIndex        =   16
         Top             =   480
         Width           =   2655
      End
      Begin MaskEdBox meCallAttempts 
      End
      Begin OptionButton optRunNow 
         BackColor       =   &H00C0C0C0&
         Caption         =   "Run Now"
         Height          =   255
         Left            =   120
         TabIndex        =   14
         Top             =   2040
         Value           =   -1  'True
         Width           =   1215
      End
      Begin CheckBox chkUnpolled 
         BackColor       =   &H00C0C0C0&
         Caption         =   "Call unpolled phones only"
         Height          =   255
         Left            =   3000
         TabIndex        =   13
         Top             =   2040
         Width           =   2535
      End
      Begin Label Label3 
         Alignment       =   2   'Center
         AutoSize        =   -1  'True
         BackStyle       =   0   'Transparent
         Caption         =   "Batch File List"
         ForeColor       =   &H00FF0000&
         Height          =   195
         Left            =   1320
         TabIndex        =   18
         Top             =   960
         Width           =   1245
      End
      Begin Label Label1 
         Alignment       =   2   'Center
         AutoSize        =   -1  'True
         BackColor       =   &H00C0C0C0&
         Caption         =   "Current File Name"
         ForeColor       =   &H00FF0000&
         Height          =   195
         Left            =   1200
         TabIndex        =   5
         Top             =   240
         Width           =   1545
      End
      Begin Label Label2 
         Alignment       =   2   'Center
         AutoSize        =   -1  'True
         BackColor       =   &H00C0C0C0&
         Caption         =   "Call Attempts"
         ForeColor       =   &H00FF0000&
         Height          =   195
         Left            =   3810
         TabIndex        =   11
         Top             =   480
         Width           =   1155
      End
   End
   Begin Label lblDateTime 
      AutoSize        =   -1  'True
      BackStyle       =   0   'Transparent
      Caption         =   "Current time is now:  mm-dd-yyyy hh:mm:ss AM"
      Height          =   195
      Left            =   360
      TabIndex        =   10
      Top             =   2640
      Width           =   3945
   End
   Begin Label lblStatus 
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      Height          =   195
      Left            =   120
      TabIndex        =   9
      Top             =   5520
      Visible         =   0   'False
      Width           =   75
   End
End

