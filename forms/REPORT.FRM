VERSION 2.00
Begin Form frmReport 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   Caption         =   "Report Generator"
   ClientHeight    =   5010
   ClientLeft      =   105
   ClientTop       =   1995
   ClientWidth     =   9480
   FontBold        =   -1  'True
   FontItalic      =   0   'False
   FontName        =   "Courier"
   FontSize        =   9.75
   FontStrikethru  =   0   'False
   FontUnderline   =   0   'False
   Height          =   5520
   Left            =   45
   LinkTopic       =   "Form1"
   MaxButton       =   0   'False
   ScaleHeight     =   5010
   ScaleWidth      =   9480
   Top             =   1545
   Width           =   9600
   Begin ListBox List5 
      FontBold        =   -1  'True
      FontItalic      =   0   'False
      FontName        =   "Courier New"
      FontSize        =   9
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   30
      Left            =   7920
      TabIndex        =   22
      Top             =   480
      Visible         =   0   'False
      Width           =   1215
   End
   Begin FileListBox File1 
      Height          =   420
      Left            =   5040
      TabIndex        =   21
      Top             =   4920
      Visible         =   0   'False
      Width           =   1215
   End
   Begin ListBox List4 
      ForeColor       =   &H00000000&
      Height          =   225
      Left            =   6840
      TabIndex        =   20
      Top             =   4320
      Visible         =   0   'False
      Width           =   2055
   End
   Begin ListBox List3 
      ForeColor       =   &H00000000&
      Height          =   225
      Left            =   3480
      TabIndex        =   19
      Top             =   4320
      Visible         =   0   'False
      Width           =   3255
   End
   Begin ListBox List2 
      BackColor       =   &H0000FFFF&
      FontBold        =   0   'False
      FontItalic      =   0   'False
      FontName        =   "Courier New"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      ForeColor       =   &H00000000&
      Height          =   3180
      Left            =   2520
      TabIndex        =   18
      Top             =   1080
      Visible         =   0   'False
      Width           =   4935
   End
   Begin MaskEdBox edtDateRange 
   End
   Begin MaskEdBox edtDateRange 
   End
   Begin CommandButton Command7 
      Caption         =   "Clear &List Box"
      Height          =   375
      Left            =   6000
      TabIndex        =   6
      Top             =   4560
      Width           =   1575
   End
   Begin CommandButton Command6 
      Caption         =   "&Daily"
      Height          =   380
      Left            =   120
      TabIndex        =   7
      Top             =   4080
      Visible         =   0   'False
      Width           =   1215
   End
   Begin CommandButton Command5 
      Caption         =   "Coin &Box Graph"
      Height          =   375
      Left            =   2160
      TabIndex        =   8
      Top             =   4560
      Width           =   1695
   End
   Begin CommandButton Command4 
      Caption         =   "&Cumulative Graph"
      Height          =   372
      Left            =   120
      TabIndex        =   13
      Top             =   4560
      Width           =   1812
   End
   Begin CommandButton Command3 
      Caption         =   "Grap&h"
      Height          =   375
      Left            =   1560
      TabIndex        =   11
      Top             =   4080
      Visible         =   0   'False
      Width           =   975
   End
   Begin CommandButton Command2 
      Caption         =   "Clear Gra&ph"
      Height          =   384
      Left            =   7800
      TabIndex        =   4
      Top             =   4560
      Width           =   1332
   End
   Begin CommandButton Command1 
      Caption         =   "Generate &Report"
      Height          =   372
      Left            =   4080
      TabIndex        =   3
      Top             =   4560
      Width           =   1692
   End
   Begin ListBox List1 
      BackColor       =   &H00FFFFFF&
      FontBold        =   0   'False
      FontItalic      =   0   'False
      FontName        =   "Courier New"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   3180
      Left            =   120
      TabIndex        =   2
      Top             =   1080
      Width           =   9255
   End
   Begin SpinButton Spin2 
   End
   Begin SpinButton Spin1 
   End
   Begin GRAPH Graph1 
   End
   Begin Label lblDays 
      Alignment       =   2   'Center
      BackColor       =   &H00808080&
      BorderStyle     =   1   'Fixed Single
      Caption         =   "0"
      ForeColor       =   &H00FFFFFF&
      Height          =   252
      Left            =   6600
      TabIndex        =   17
      Top             =   480
      Width           =   612
   End
   Begin Label lblCurrentPhoneNo 
      Alignment       =   2   'Center
      BackColor       =   &H00808080&
      BorderStyle     =   1   'Fixed Single
      Caption         =   "770-448-7788"
      ForeColor       =   &H00FFFFFF&
      Height          =   255
      Left            =   1560
      TabIndex        =   14
      Top             =   120
      Width           =   1935
   End
   Begin Label Label10 
      Alignment       =   1   'Right Justify
      BackColor       =   &H00C0C0C0&
      Caption         =   "Working File:"
      Height          =   255
      Left            =   120
      TabIndex        =   9
      Top             =   480
      Width           =   1335
   End
   Begin Label Label9 
      BackColor       =   &H00C0C0C0&
      Height          =   255
      Left            =   1560
      TabIndex        =   10
      Top             =   480
      Width           =   2895
   End
   Begin Label lblHead 
      BackColor       =   &H00C0C0C0&
      FontBold        =   0   'False
      FontItalic      =   0   'False
      FontName        =   "Courier New"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   255
      Left            =   180
      TabIndex        =   5
      Top             =   840
      Width           =   9135
   End
   Begin Line Line4 
      X1              =   7680
      X2              =   7200
      Y1              =   600
      Y2              =   600
   End
   Begin Line Line3 
      X1              =   6120
      X2              =   6600
      Y1              =   600
      Y2              =   600
   End
   Begin Line Line2 
      X1              =   7680
      X2              =   7680
      Y1              =   360
      Y2              =   600
   End
   Begin Line Line1 
      X1              =   6120
      X2              =   6120
      Y1              =   240
      Y2              =   600
   End
   Begin Label Label2 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Phone Number:"
      Height          =   255
      Left            =   120
      TabIndex        =   1
      Top             =   120
      Width           =   1335
   End
   Begin Label Label1 
      Alignment       =   2   'Center
      BackColor       =   &H00C0C0C0&
      Caption         =   "To"
      Height          =   252
      Left            =   6600
      TabIndex        =   0
      Top             =   120
      Width           =   612
   End
End

