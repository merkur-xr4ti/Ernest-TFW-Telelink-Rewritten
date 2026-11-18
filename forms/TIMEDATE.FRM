VERSION 2.00
Begin Form frmTimeDate 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   Caption         =   "Time & Date"
   ClientHeight    =   2565
   ClientLeft      =   1125
   ClientTop       =   2145
   ClientWidth     =   3885
   Height          =   3075
   Left            =   1065
   LinkTopic       =   "Form9"
   MaxButton       =   0   'False
   ScaleHeight     =   2565
   ScaleWidth      =   3885
   Top             =   1695
   Width           =   4005
   Begin CommandButton cmdNow 
      Caption         =   "Current Date && Time"
      Height          =   495
      Left            =   1080
      TabIndex        =   6
      Top             =   2040
      Width           =   1935
   End
   Begin ComboBox Combo1 
      Height          =   320
      Left            =   1920
      Style           =   2   'Dropdown List
      TabIndex        =   5
      Top             =   360
      Width           =   1455
   End
   Begin MaskEdBox MaskedEdit2 
   End
   Begin MaskEdBox MaskedEdit1 
   End
   Begin Label Label3 
      Alignment       =   1   'Right Justify
      BackColor       =   &H00C0C0C0&
      Caption         =   "Day"
      Height          =   260
      Left            =   240
      TabIndex        =   2
      Top             =   360
      Width           =   1215
   End
   Begin Label Label2 
      Alignment       =   1   'Right Justify
      BackColor       =   &H00C0C0C0&
      Caption         =   "Date"
      Height          =   260
      Left            =   600
      TabIndex        =   1
      Top             =   960
      Width           =   855
   End
   Begin Label Label1 
      Alignment       =   1   'Right Justify
      BackColor       =   &H00C0C0C0&
      Caption         =   "Time"
      Height          =   260
      Left            =   360
      TabIndex        =   0
      Top             =   1560
      Width           =   1095
   End
End

