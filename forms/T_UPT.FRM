VERSION 2.00
Begin Form frmUPTTable 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   Caption         =   "UPT Table"
   ClientHeight    =   4965
   ClientLeft      =   825
   ClientTop       =   1605
   ClientWidth     =   7380
   Height          =   5475
   Left            =   765
   LinkTopic       =   "Form4"
   MaxButton       =   0   'False
   ScaleHeight     =   4965
   ScaleWidth      =   7380
   Top             =   1155
   Width           =   7500
   Begin ListBox List3 
      Height          =   225
      Left            =   4920
      TabIndex        =   13
      Top             =   4800
      Visible         =   0   'False
      Width           =   975
   End
   Begin CommandButton cmdEdit 
      Caption         =   "&Edit"
      Enabled         =   0   'False
      Height          =   500
      Left            =   2520
      TabIndex        =   9
      Top             =   4320
      Width           =   975
   End
   Begin CommandButton cmdAdd 
      Caption         =   "&Add"
      Height          =   500
      Left            =   1320
      TabIndex        =   8
      Top             =   4320
      Width           =   975
   End
   Begin ListBox List2 
      Height          =   225
      Left            =   3240
      Sorted          =   -1  'True
      TabIndex        =   2
      Top             =   4800
      Visible         =   0   'False
      Width           =   975
   End
   Begin ListBox List1 
      Height          =   3150
      Left            =   120
      TabIndex        =   4
      Top             =   480
      Width           =   7155
   End
   Begin CommandButton cmdClear 
      Caption         =   "&Clear"
      Enabled         =   0   'False
      Height          =   500
      Left            =   4920
      TabIndex        =   5
      Top             =   4320
      Width           =   975
   End
   Begin CommandButton cmdDelete 
      Caption         =   "&Delete"
      Enabled         =   0   'False
      Height          =   500
      Left            =   3720
      TabIndex        =   7
      Top             =   4320
      Width           =   975
   End
   Begin Label lblChange 
      BackStyle       =   0   'Transparent
      Caption         =   "Don't forget to save your changes!"
      ForeColor       =   &H000000FF&
      Height          =   255
      Left            =   4320
      TabIndex        =   12
      Tag             =   "No"
      Top             =   3840
      Visible         =   0   'False
      Width           =   3015
   End
   Begin Label lblCurrFile 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Working File:"
      Height          =   255
      Left            =   180
      TabIndex        =   11
      Top             =   3840
      Width           =   1215
   End
   Begin Label lblWorkingFile 
      BackColor       =   &H00C0C0C0&
      Height          =   255
      Left            =   1440
      TabIndex        =   10
      Top             =   3840
      Width           =   2895
   End
   Begin Label Label7 
      BackColor       =   &H00C0C0C0&
      Caption         =   "2nd $/Time"
      Height          =   255
      Left            =   5040
      TabIndex        =   6
      Top             =   240
      Width           =   1095
   End
   Begin Label Label4 
      BackColor       =   &H00C0C0C0&
      Caption         =   "1st $/Time"
      Height          =   255
      Left            =   3480
      TabIndex        =   3
      Top             =   240
      Width           =   975
   End
   Begin Label Label2 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Rate Type"
      Height          =   255
      Left            =   1800
      TabIndex        =   1
      Top             =   240
      Width           =   975
   End
   Begin Label Label1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Phone Number"
      Height          =   252
      Left            =   180
      TabIndex        =   0
      Top             =   240
      Width           =   1392
   End
End

