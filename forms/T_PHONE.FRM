VERSION 2.00
Begin Form frmPhoneList 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   Caption         =   "Phone List"
   ClientHeight    =   4995
   ClientLeft      =   240
   ClientTop       =   3330
   ClientWidth     =   9030
   FontBold        =   -1  'True
   FontItalic      =   0   'False
   FontName        =   "System"
   FontSize        =   9.75
   FontStrikethru  =   0   'False
   FontUnderline   =   0   'False
   Height          =   5505
   Left            =   180
   LinkTopic       =   "Form1"
   MaxButton       =   0   'False
   ScaleHeight     =   4995
   ScaleWidth      =   9030
   Top             =   2880
   Width           =   9150
   Begin OptionButton optShowAll 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Show all"
      Height          =   255
      Left            =   4560
      TabIndex        =   22
      Top             =   3960
      Width           =   2655
   End
   Begin OptionButton optShowU 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Show unsuccessful polls only"
      Height          =   255
      Left            =   1560
      TabIndex        =   21
      Top             =   3960
      Width           =   2895
   End
   Begin CommandButton Command1 
      Caption         =   "&Sort"
      Height          =   495
      Left            =   7680
      TabIndex        =   20
      Top             =   4320
      Width           =   1095
   End
   Begin Grid GridX 
   End
   Begin ListBox lstSortGrid 
      Height          =   225
      Left            =   8160
      Sorted          =   -1  'True
      TabIndex        =   18
      Top             =   3840
      Visible         =   0   'False
      Width           =   615
   End
   Begin Grid Grid2 
   End
   Begin MaskEdBox meIDBox 
   End
   Begin TextBox txtPhoneNumber 
      Height          =   300
      Left            =   1500
      TabIndex        =   13
      Top             =   480
      Width           =   2115
   End
   Begin CommandButton Command3 
      Caption         =   "Search"
      Height          =   375
      Left            =   3720
      TabIndex        =   12
      Top             =   480
      Width           =   855
   End
   Begin CommandButton Command4 
      Caption         =   "Search"
      Height          =   375
      Left            =   8040
      TabIndex        =   11
      Top             =   480
      Width           =   855
   End
   Begin CommandButton cmdRunBatch 
      Caption         =   "&Re-Run Batch"
      Enabled         =   0   'False
      Height          =   495
      Left            =   6000
      TabIndex        =   4
      Top             =   4320
      Width           =   1455
   End
   Begin CommandButton cmdClear 
      Caption         =   "&Clear"
      Enabled         =   0   'False
      Height          =   495
      Left            =   4800
      TabIndex        =   7
      Top             =   4320
      Width           =   1095
   End
   Begin CommandButton cmdDelete 
      Caption         =   "&Delete"
      Enabled         =   0   'False
      Height          =   500
      Left            =   3660
      TabIndex        =   2
      Top             =   4320
      Width           =   975
   End
   Begin CommandButton cmdEdit 
      Caption         =   "&Edit"
      Enabled         =   0   'False
      Height          =   500
      Left            =   2460
      TabIndex        =   1
      Top             =   4320
      Width           =   975
   End
   Begin CommandButton cmdAdd 
      Caption         =   "&Add"
      Height          =   500
      Left            =   1260
      TabIndex        =   0
      Top             =   4320
      Width           =   975
   End
   Begin CommandButton cmdTag 
      Caption         =   "&Tag for Re-Run"
      Height          =   495
      Left            =   4320
      TabIndex        =   5
      Top             =   4320
      Width           =   1455
   End
   Begin Grid Grid1 
   End
   Begin Label Label1 
      AutoSize        =   -1  'True
      BackColor       =   &H8000000F&
      Caption         =   "0"
      Height          =   195
      Left            =   7800
      TabIndex        =   16
      Top             =   4080
      Visible         =   0   'False
      Width           =   120
   End
   Begin Label Label3 
      Alignment       =   1   'Right Justify
      BackColor       =   &H00C0C0C0&
      Caption         =   "Phone Number:"
      Height          =   255
      Left            =   120
      TabIndex        =   15
      Top             =   480
      Width           =   1335
   End
   Begin Label Label13 
      Alignment       =   1   'Right Justify
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      Caption         =   "ID Number:"
      Height          =   195
      Left            =   5640
      TabIndex        =   14
      Top             =   480
      Width           =   975
   End
   Begin Label lblChange 
      BackColor       =   &H8000000F&
      BackStyle       =   0   'Transparent
      Caption         =   "<-- Don't forget to save your changes!"
      ForeColor       =   &H000000FF&
      Height          =   255
      Left            =   4440
      TabIndex        =   6
      Tag             =   "No"
      Top             =   120
      Visible         =   0   'False
      Width           =   3375
   End
   Begin Label lblCurrFile 
      Alignment       =   1   'Right Justify
      BackColor       =   &H00C0C0C0&
      Caption         =   "Working File:"
      Height          =   255
      Left            =   120
      TabIndex        =   9
      Top             =   120
      Width           =   1215
   End
   Begin Label LblWorkingFile 
      BackColor       =   &H00C0C0C0&
      Height          =   255
      Left            =   1440
      TabIndex        =   8
      Top             =   120
      Width           =   2895
   End
End

