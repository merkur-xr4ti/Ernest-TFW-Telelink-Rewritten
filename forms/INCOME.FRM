VERSION 2.00
Begin Form frmIncome 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   ClientHeight    =   4470
   ClientLeft      =   1185
   ClientTop       =   1950
   ClientWidth     =   7815
   Height          =   4980
   Left            =   1125
   LinkTopic       =   "Form1"
   MaxButton       =   0   'False
   ScaleHeight     =   4470
   ScaleWidth      =   7815
   Top             =   1500
   Width           =   7935
   Begin MaskEdBox meIDNumber 
   End
   Begin CommandButton Command4 
      Caption         =   "Search"
      Height          =   375
      Left            =   5400
      TabIndex        =   19
      Top             =   600
      Width           =   855
   End
   Begin CommandButton Command3 
      Caption         =   "Search"
      Height          =   375
      Left            =   5400
      TabIndex        =   18
      Top             =   120
      Width           =   855
   End
   Begin TextBox txtPhoneNumber 
      Height          =   300
      Left            =   3240
      TabIndex        =   17
      Top             =   120
      Width           =   2055
   End
   Begin TextBox Text5 
      Alignment       =   2   'Center
      Height          =   300
      Left            =   2760
      TabIndex        =   15
      Top             =   1920
      Width           =   495
   End
   Begin CommandButton Command2 
      Caption         =   "&Next"
      Height          =   500
      Left            =   1800
      TabIndex        =   13
      Top             =   3480
      Visible         =   0   'False
      Width           =   1215
   End
   Begin CommandButton Command1 
      Caption         =   "&Previous"
      Height          =   500
      Left            =   240
      TabIndex        =   12
      Top             =   3480
      Visible         =   0   'False
      Width           =   1215
   End
   Begin ListBox List2 
      Height          =   1200
      Left            =   3960
      TabIndex        =   11
      Top             =   2640
      Width           =   3615
   End
   Begin TextBox Text4 
      Height          =   300
      Left            =   1800
      TabIndex        =   8
      Top             =   2880
      Width           =   1335
   End
   Begin TextBox Text3 
      Height          =   300
      Left            =   1800
      TabIndex        =   7
      Top             =   2400
      Width           =   1815
   End
   Begin TextBox Text2 
      Height          =   300
      Left            =   1800
      TabIndex        =   6
      Top             =   1920
      Width           =   855
   End
   Begin TextBox Text1 
      Height          =   300
      Left            =   1800
      TabIndex        =   5
      Top             =   1440
      Width           =   1455
   End
   Begin ListBox List1 
      Height          =   810
      Left            =   3960
      TabIndex        =   4
      Top             =   1440
      Width           =   3615
   End
   Begin Label Label13 
      Alignment       =   1   'Right Justify
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      Caption         =   "ID Number:"
      Height          =   195
      Left            =   2160
      TabIndex        =   21
      Top             =   600
      Width           =   975
   End
   Begin Label Label9 
      Alignment       =   1   'Right Justify
      BackColor       =   &H00C0C0C0&
      Caption         =   "Phone Number:"
      Height          =   255
      Left            =   1800
      TabIndex        =   20
      Top             =   120
      Width           =   1335
   End
   Begin Label Label8 
      BackColor       =   &H00C0C0C0&
      Height          =   255
      Left            =   3960
      TabIndex        =   16
      Top             =   4080
      Width           =   3735
   End
   Begin Label Label7 
      Alignment       =   2   'Center
      BackColor       =   &H00C0C0C0&
      Height          =   255
      Left            =   240
      TabIndex        =   14
      Top             =   4080
      Width           =   2775
   End
   Begin Label Label6 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Active Status Flags"
      Height          =   255
      Left            =   3960
      TabIndex        =   10
      Top             =   2400
      Width           =   1695
   End
   Begin Label Label5 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Coin Box Status"
      Height          =   255
      Left            =   3960
      TabIndex        =   9
      Top             =   1200
      Width           =   1455
   End
   Begin Label Label4 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Coin Box Amount"
      Height          =   255
      Left            =   240
      TabIndex        =   3
      Top             =   2880
      Width           =   1575
   End
   Begin Label Label3 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Clock Data"
      Height          =   255
      Left            =   720
      TabIndex        =   2
      Top             =   2400
      Width           =   975
   End
   Begin Label Label2 
      BackColor       =   &H00C0C0C0&
      Caption         =   "ID Number"
      Height          =   255
      Left            =   720
      TabIndex        =   1
      Top             =   1920
      Width           =   975
   End
   Begin Label Label1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Phone Number"
      Height          =   255
      Left            =   360
      TabIndex        =   0
      Top             =   1440
      Width           =   1335
   End
End

