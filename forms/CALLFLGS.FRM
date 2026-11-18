VERSION 2.00
Begin Form frmCallFlags 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   Caption         =   "Call Flags"
   ClientHeight    =   4545
   ClientLeft      =   465
   ClientTop       =   1695
   ClientWidth     =   8640
   Height          =   5055
   Left            =   405
   LinkTopic       =   "Form1"
   MaxButton       =   0   'False
   ScaleHeight     =   4545
   ScaleWidth      =   8640
   Top             =   1245
   Width           =   8760
   Begin SSRibbon gbtnSortCol 
   End
   Begin SSRibbon gbtnSortCol 
   End
   Begin SSRibbon gbtnSortCol 
   End
   Begin SSRibbon gbtnSortCol 
   End
   Begin ListBox List4Sort 
      Height          =   225
      Left            =   7080
      Sorted          =   -1  'True
      TabIndex        =   10
      Top             =   -60
      Visible         =   0   'False
      Width           =   1515
   End
   Begin MaskEdBox meIDBox 
   End
   Begin CommandButton Command4 
      Caption         =   "Search"
      Height          =   315
      Left            =   7020
      TabIndex        =   6
      Top             =   4260
      Visible         =   0   'False
      Width           =   855
   End
   Begin CommandButton Command3 
      Caption         =   "Search"
      Height          =   315
      Left            =   7020
      TabIndex        =   5
      Top             =   3960
      Visible         =   0   'False
      Width           =   855
   End
   Begin TextBox txtPhoneNumber 
      Height          =   300
      Left            =   4800
      TabIndex        =   4
      Top             =   3960
      Visible         =   0   'False
      Width           =   2175
   End
   Begin CommandButton cmdCancel 
      Cancel          =   -1  'True
      Caption         =   "&Cancel"
      Height          =   375
      Left            =   1680
      TabIndex        =   2
      Top             =   4020
      Width           =   1095
   End
   Begin CommandButton cmdClear 
      Caption         =   "&Save && Clear"
      Enabled         =   0   'False
      Height          =   375
      Left            =   180
      TabIndex        =   1
      Top             =   4020
      Width           =   1335
   End
   Begin ListBox List4Display 
      FontBold        =   0   'False
      FontItalic      =   0   'False
      FontName        =   "Terminal"
      FontSize        =   9
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   3270
      Left            =   60
      TabIndex        =   0
      Top             =   420
      Width           =   8535
   End
   Begin Label Label13 
      Alignment       =   1   'Right Justify
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      Caption         =   "ID Number:"
      Height          =   195
      Left            =   3720
      TabIndex        =   8
      Top             =   4320
      Visible         =   0   'False
      Width           =   975
   End
   Begin Label Label3 
      Alignment       =   1   'Right Justify
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      Caption         =   "Phone Number:"
      Height          =   195
      Left            =   3420
      TabIndex        =   7
      Top             =   4020
      Visible         =   0   'False
      Width           =   1320
   End
   Begin Label lblWorkingFile 
      AutoSize        =   -1  'True
      BackStyle       =   0   'Transparent
      Height          =   195
      Left            =   60
      TabIndex        =   3
      Top             =   0
      Width           =   2355
   End
End

