VERSION 2.00
Begin Form frmGetProgrammableFeatures 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   Caption         =   "105"
   ClientHeight    =   3720
   ClientLeft      =   2595
   ClientTop       =   1500
   ClientWidth     =   5985
   Height          =   4230
   Left            =   2535
   LinkTopic       =   "Form1"
   MaxButton       =   0   'False
   ScaleHeight     =   3720
   ScaleWidth      =   5985
   Top             =   1050
   Width           =   6105
   Begin MaskEdBox MaskedEdit2 
   End
   Begin MaskEdBox MaskedEdit1 
   End
   Begin CommandButton Command2 
      Cancel          =   -1  'True
      Caption         =   "&Cancel"
      Height          =   500
      Left            =   4320
      TabIndex        =   2
      Top             =   2760
      Width           =   975
   End
   Begin CommandButton Command1 
      Caption         =   "&OK"
      Default         =   -1  'True
      Height          =   500
      Left            =   4320
      TabIndex        =   1
      Top             =   2040
      Width           =   975
   End
   Begin Frame Frame1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Value"
      Height          =   1700
      Left            =   600
      TabIndex        =   5
      Top             =   1800
      Width           =   2535
      Begin OptionButton Option1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "Option1"
         Height          =   260
         Index           =   3
         Left            =   720
         TabIndex        =   9
         Top             =   1320
         Width           =   1215
      End
      Begin OptionButton Option1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "Option1"
         Height          =   260
         Index           =   2
         Left            =   720
         TabIndex        =   8
         Top             =   960
         Width           =   1215
      End
      Begin OptionButton Option1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "Option1"
         Height          =   260
         Index           =   1
         Left            =   720
         TabIndex        =   7
         Top             =   600
         Width           =   1215
      End
      Begin OptionButton Option1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "Option1"
         Height          =   260
         Index           =   0
         Left            =   720
         TabIndex        =   6
         Top             =   240
         Width           =   1215
      End
   End
   Begin Label Label3 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Old NPA"
      Height          =   260
      Left            =   960
      TabIndex        =   10
      Top             =   2760
      Width           =   735
   End
   Begin Label Label2 
      Alignment       =   1   'Right Justify
      BackColor       =   &H00C0C0C0&
      Caption         =   "Value"
      Height          =   260
      Left            =   840
      TabIndex        =   3
      Top             =   2280
      Width           =   855
   End
   Begin Label Label1 
      BorderStyle     =   1   'Fixed Single
      Height          =   1100
      Left            =   600
      TabIndex        =   0
      Top             =   360
      Width           =   4695
      WordWrap        =   -1  'True
   End
End

