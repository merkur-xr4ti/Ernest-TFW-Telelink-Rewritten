VERSION 2.00
Begin Form frmAOSScript 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   Caption         =   "AOS Scripts"
   ClientHeight    =   4185
   ClientLeft      =   2475
   ClientTop       =   2280
   ClientWidth     =   6690
   Height          =   4695
   Left            =   2415
   LinkTopic       =   "Form1"
   MaxButton       =   0   'False
   ScaleHeight     =   4185
   ScaleWidth      =   6690
   Top             =   1830
   Width           =   6810
   Begin MaskEdBox MaskedEdit2 
   End
   Begin MaskEdBox MaskedEdit1 
   End
   Begin ListBox List2 
      Height          =   1200
      Index           =   3
      Left            =   6120
      TabIndex        =   14
      Top             =   2760
      Visible         =   0   'False
      Width           =   495
   End
   Begin ListBox List2 
      Height          =   1200
      Index           =   2
      Left            =   6120
      TabIndex        =   13
      Top             =   1440
      Visible         =   0   'False
      Width           =   495
   End
   Begin ListBox List2 
      Height          =   1200
      Index           =   1
      Left            =   6120
      TabIndex        =   12
      Top             =   120
      Visible         =   0   'False
      Width           =   495
   End
   Begin CommandButton Command4 
      Caption         =   "In&sert"
      Enabled         =   0   'False
      Height          =   500
      Left            =   4440
      TabIndex        =   11
      Top             =   2400
      Width           =   1095
   End
   Begin ListBox List1 
      Height          =   1590
      Index           =   3
      Left            =   240
      TabIndex        =   10
      Top             =   960
      Visible         =   0   'False
      Width           =   2955
   End
   Begin ListBox List1 
      Height          =   1590
      Index           =   2
      Left            =   240
      TabIndex        =   9
      Top             =   960
      Visible         =   0   'False
      Width           =   2895
   End
   Begin CommandButton Command3 
      Caption         =   "Clear"
      Enabled         =   0   'False
      Height          =   500
      Left            =   4440
      TabIndex        =   5
      Top             =   3600
      Width           =   1095
   End
   Begin CommandButton Command2 
      Caption         =   "&Remove"
      Enabled         =   0   'False
      Height          =   500
      Left            =   4440
      TabIndex        =   4
      Top             =   3000
      Width           =   1095
   End
   Begin CommandButton Command1 
      Caption         =   "&Add"
      Enabled         =   0   'False
      Height          =   500
      Left            =   4440
      TabIndex        =   3
      Top             =   1800
      Width           =   1095
   End
   Begin Frame Frame1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Active Script"
      Height          =   1460
      Left            =   3600
      TabIndex        =   2
      Top             =   120
      Width           =   2295
      Begin OptionButton Option3 
         BackColor       =   &H00C0C0C0&
         Caption         =   "AOS Script 3"
         Height          =   260
         Left            =   480
         TabIndex        =   8
         Top             =   1080
         Width           =   1455
      End
      Begin OptionButton Option2 
         BackColor       =   &H00C0C0C0&
         Caption         =   "AOS Script 2"
         Height          =   260
         Left            =   480
         TabIndex        =   7
         Top             =   720
         Width           =   1455
      End
      Begin OptionButton Option1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "AOS Script 1"
         Height          =   260
         Left            =   480
         TabIndex        =   6
         Top             =   360
         Value           =   -1  'True
         Width           =   1455
      End
   End
   Begin ListBox List1 
      Height          =   1590
      Index           =   1
      Left            =   240
      TabIndex        =   0
      Top             =   960
      Width           =   2895
   End
   Begin Label lblChange 
      Alignment       =   2   'Center
      AutoSize        =   -1  'True
      BackStyle       =   0   'Transparent
      Caption         =   "Don't forget to save your changes!"
      ForeColor       =   &H000000FF&
      Height          =   195
      Left            =   675
      TabIndex        =   21
      Tag             =   "No"
      Top             =   3000
      Visible         =   0   'False
      Width           =   2985
   End
   Begin Label lblCurrFile 
      BackColor       =   &H00C0C0C0&
      Height          =   255
      Left            =   240
      TabIndex        =   20
      Top             =   2760
      Width           =   3855
   End
   Begin Label Label5 
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      Caption         =   "Auth. Code"
      Height          =   195
      Left            =   240
      TabIndex        =   19
      Top             =   3720
      Width           =   960
   End
   Begin Label Label4 
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      Caption         =   "Phone #"
      Height          =   195
      Left            =   240
      TabIndex        =   17
      Top             =   3360
      Width           =   735
   End
   Begin Label Label3 
      BorderStyle     =   1   'Fixed Single
      Height          =   255
      Left            =   240
      TabIndex        =   15
      Top             =   480
      Width           =   2895
   End
   Begin Label Label1 
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      Caption         =   "Instruction to Add..."
      Height          =   195
      Left            =   240
      TabIndex        =   1
      Top             =   240
      Width           =   1710
   End
End

