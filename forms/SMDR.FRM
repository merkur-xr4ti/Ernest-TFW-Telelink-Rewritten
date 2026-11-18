VERSION 2.00
Begin Form frmSMDR 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   Caption         =   "SMDR"
   ClientHeight    =   4305
   ClientLeft      =   -135
   ClientTop       =   2325
   ClientWidth     =   8550
   Height          =   4815
   Left            =   -195
   LinkTopic       =   "Form7"
   MaxButton       =   0   'False
   ScaleHeight     =   4305
   ScaleWidth      =   8550
   Top             =   1875
   Width           =   8670
   Begin ListBox lstSubTotals 
      Height          =   225
      Left            =   5040
      TabIndex        =   28
      Top             =   4320
      Visible         =   0   'False
      Width           =   1335
   End
   Begin ListBox List3 
      Height          =   225
      Left            =   2880
      TabIndex        =   27
      Top             =   4320
      Visible         =   0   'False
      Width           =   855
   End
   Begin ListBox lstGrandTotals 
      Height          =   225
      Left            =   6480
      Sorted          =   -1  'True
      TabIndex        =   26
      Top             =   4320
      Visible         =   0   'False
      Width           =   1335
   End
   Begin ListBox lstPrintList 
      Height          =   225
      Left            =   3840
      Sorted          =   -1  'True
      TabIndex        =   25
      Top             =   4320
      Visible         =   0   'False
      Width           =   1095
   End
   Begin MaskEdBox meIDBox 
   End
   Begin CommandButton Command4 
      Caption         =   "Search"
      Height          =   375
      Left            =   7320
      TabIndex        =   23
      Top             =   120
      Width           =   855
   End
   Begin CommandButton Command3 
      Caption         =   "Search"
      Height          =   375
      Left            =   3720
      TabIndex        =   22
      Top             =   120
      Width           =   855
   End
   Begin FileListBox filSMD 
      Height          =   225
      Left            =   1560
      Pattern         =   "*.smd"
      TabIndex        =   20
      Top             =   4320
      Visible         =   0   'False
      Width           =   1215
   End
   Begin ListBox List2 
      Height          =   225
      Left            =   240
      Sorted          =   -1  'True
      TabIndex        =   19
      Top             =   4320
      Visible         =   0   'False
      Width           =   1215
   End
   Begin CommandButton Command2 
      Caption         =   "Next D&ay"
      Height          =   495
      Left            =   1800
      TabIndex        =   18
      Top             =   3720
      Width           =   1215
   End
   Begin CommandButton Command1 
      Caption         =   "Previous &Day"
      Height          =   495
      Left            =   240
      TabIndex        =   17
      Top             =   3720
      Width           =   1335
   End
   Begin TextBox txtPhoneNumber 
      Height          =   300
      Left            =   1500
      TabIndex        =   1
      Top             =   120
      Width           =   2115
   End
   Begin CommandButton cmdNextPhone 
      Caption         =   "&Next Phone"
      Height          =   500
      Left            =   6600
      TabIndex        =   9
      Top             =   3720
      Width           =   1695
   End
   Begin CommandButton cmdPreviousPhone 
      Caption         =   "&Previous Phone"
      Height          =   500
      Left            =   4680
      TabIndex        =   8
      Top             =   3720
      Width           =   1695
   End
   Begin ListBox List1 
      FontBold        =   0   'False
      FontItalic      =   0   'False
      FontName        =   "MS Sans Serif"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   1980
      Left            =   240
      TabIndex        =   2
      Top             =   1440
      Width           =   8055
   End
   Begin Label Label13 
      Alignment       =   1   'Right Justify
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      Caption         =   "ID Number:"
      Height          =   195
      Left            =   4920
      TabIndex        =   21
      Top             =   120
      Width           =   975
   End
   Begin Label Label12 
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      BackStyle       =   0   'Transparent
      Caption         =   "Code"
      Height          =   195
      Left            =   4560
      TabIndex        =   16
      Top             =   1200
      Visible         =   0   'False
      Width           =   450
   End
   Begin Label Label11 
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      BackStyle       =   0   'Transparent
      Caption         =   "Posted"
      Height          =   195
      Left            =   5160
      TabIndex        =   15
      Top             =   1200
      Visible         =   0   'False
      Width           =   600
   End
   Begin Label Label10 
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      BackStyle       =   0   'Transparent
      Caption         =   "Transaction"
      Height          =   195
      Left            =   4680
      TabIndex        =   14
      Top             =   960
      Visible         =   0   'False
      Width           =   1020
   End
   Begin Label Label9 
      Alignment       =   2   'Center
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      BackStyle       =   0   'Transparent
      Caption         =   "Type of Call"
      Height          =   375
      Left            =   6120
      TabIndex        =   13
      Top             =   960
      Visible         =   0   'False
      Width           =   675
      WordWrap        =   -1  'True
   End
   Begin Label lblWorkingFile 
      BackColor       =   &H00C0C0C0&
      BackStyle       =   0   'Transparent
      Height          =   300
      Left            =   1560
      TabIndex        =   12
      Top             =   600
      Width           =   6735
   End
   Begin Label Label8 
      Alignment       =   1   'Right Justify
      BackColor       =   &H00C0C0C0&
      Caption         =   "Working File:"
      Height          =   255
      Left            =   240
      TabIndex        =   11
      Top             =   600
      Width           =   1215
   End
   Begin Label Label7 
      BackColor       =   &H00C0C0C0&
      BackStyle       =   0   'Transparent
      Caption         =   "Type of     Call"
      Height          =   375
      Left            =   5520
      TabIndex        =   10
      Top             =   960
      Width           =   735
   End
   Begin Label Label6 
      BackColor       =   &H00C0C0C0&
      BackStyle       =   0   'Transparent
      Caption         =   "Amt."
      Height          =   240
      Left            =   3840
      TabIndex        =   7
      Top             =   1200
      Width           =   735
   End
   Begin Label Label5 
      BackColor       =   &H00C0C0C0&
      BackStyle       =   0   'Transparent
      Caption         =   "Len."
      Height          =   240
      Left            =   3120
      TabIndex        =   6
      Top             =   1200
      Width           =   735
   End
   Begin Label Label4 
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      BackStyle       =   0   'Transparent
      Caption         =   "Phone #"
      Height          =   195
      Left            =   1680
      TabIndex        =   5
      Top             =   1200
      Width           =   735
   End
   Begin Label Label3 
      BackColor       =   &H00C0C0C0&
      BackStyle       =   0   'Transparent
      Caption         =   " Time"
      Height          =   240
      Left            =   960
      TabIndex        =   4
      Top             =   1200
      Width           =   495
   End
   Begin Label Label2 
      BackColor       =   &H00C0C0C0&
      BackStyle       =   0   'Transparent
      Caption         =   "Date"
      Height          =   240
      Left            =   240
      TabIndex        =   3
      Top             =   1200
      Width           =   495
   End
   Begin Label Label1 
      Alignment       =   1   'Right Justify
      BackColor       =   &H00C0C0C0&
      Caption         =   "Phone Number:"
      Height          =   255
      Left            =   120
      TabIndex        =   0
      Top             =   120
      Width           =   1335
   End
End

