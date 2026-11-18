VERSION 2.00
Begin Form frmIncomingSchedule 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   Caption         =   "Incoming Call Schedule Setup"
   ClientHeight    =   2625
   ClientLeft      =   570
   ClientTop       =   2835
   ClientWidth     =   6225
   Height          =   3135
   Left            =   510
   LinkTopic       =   "Form1"
   MaxButton       =   0   'False
   ScaleHeight     =   2625
   ScaleWidth      =   6225
   Top             =   2385
   Width           =   6345
   Begin CheckBox chkStartup 
      BackColor       =   &H00C0C0C0&
      Caption         =   " Turn AutoAnswer on when Telelink starts"
      Height          =   255
      Left            =   120
      TabIndex        =   13
      Top             =   0
      Width           =   3975
   End
   Begin MaskEdBox edtSchedTime 
   End
   Begin MaskEdBox edtSchedTime 
   End
   Begin MaskEdBox edtSchedTime 
   End
   Begin MaskEdBox edtSchedTime 
   End
   Begin CheckBox chkAutoAnswer 
      Alignment       =   1   'Right Justify
      BackColor       =   &H00C0C0C0&
      Caption         =   "Accept D1 Incoming Calls"
      Height          =   255
      Index           =   1
      Left            =   120
      TabIndex        =   3
      Top             =   1440
      Width           =   2655
   End
   Begin CheckBox chkAutoAnswer 
      Alignment       =   1   'Right Justify
      BackColor       =   &H00C0C0C0&
      Caption         =   "Accept D3 Incoming Calls"
      Height          =   255
      Index           =   0
      Left            =   120
      TabIndex        =   0
      Top             =   840
      Width           =   2655
   End
   Begin CommandButton cmdOk 
      Caption         =   "&Ok"
      Default         =   -1  'True
      Height          =   375
      Left            =   2040
      TabIndex        =   6
      Top             =   2040
      Width           =   855
   End
   Begin CommandButton cmdCancel 
      Cancel          =   -1  'True
      Caption         =   "&Cancel"
      Height          =   375
      Left            =   3360
      TabIndex        =   7
      Top             =   2040
      Width           =   855
   End
   Begin Label lblAutoAnswer 
      Alignment       =   2   'Center
      BackStyle       =   0   'Transparent
      Caption         =   "Military Time (HHMM)"
      ForeColor       =   &H00FF0000&
      Height          =   255
      Index           =   4
      Left            =   3840
      TabIndex        =   12
      Top             =   480
      Visible         =   0   'False
      Width           =   2055
   End
   Begin Label lblAutoAnswer 
      Alignment       =   2   'Center
      BackStyle       =   0   'Transparent
      Caption         =   "and"
      Height          =   255
      Index           =   3
      Left            =   4560
      TabIndex        =   8
      Top             =   1440
      Visible         =   0   'False
      Width           =   615
   End
   Begin Label lblAutoAnswer 
      Alignment       =   2   'Center
      BackStyle       =   0   'Transparent
      Caption         =   "and"
      Height          =   255
      Index           =   1
      Left            =   4560
      TabIndex        =   9
      Top             =   840
      Visible         =   0   'False
      Width           =   615
   End
   Begin Label lblAutoAnswer 
      Alignment       =   2   'Center
      BackStyle       =   0   'Transparent
      Caption         =   "between"
      Height          =   255
      Index           =   2
      Left            =   2880
      TabIndex        =   10
      Top             =   1440
      Visible         =   0   'False
      Width           =   975
   End
   Begin Label lblAutoAnswer 
      Alignment       =   2   'Center
      BackStyle       =   0   'Transparent
      Caption         =   "between"
      Height          =   255
      Index           =   0
      Left            =   2880
      TabIndex        =   11
      Top             =   840
      Visible         =   0   'False
      Width           =   975
   End
End

