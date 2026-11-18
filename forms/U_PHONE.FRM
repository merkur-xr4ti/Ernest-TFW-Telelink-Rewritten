VERSION 2.00
Begin Form frmUpdatePhoneListEntry 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   Caption         =   "Phone List Entry"
   ClientHeight    =   3135
   ClientLeft      =   2790
   ClientTop       =   2670
   ClientWidth     =   6705
   Height          =   3645
   Left            =   2730
   LinkTopic       =   "Form2"
   MaxButton       =   0   'False
   ScaleHeight     =   3135
   ScaleWidth      =   6705
   Top             =   2220
   Width           =   6825
   Begin TextBox Zip 
      FontBold        =   0   'False
      FontItalic      =   0   'False
      FontName        =   "MS Sans Serif"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   285
      Left            =   5760
      MaxLength       =   5
      TabIndex        =   10
      Text            =   "Text4"
      Top             =   960
      Width           =   735
   End
   Begin TextBox State 
      FontBold        =   0   'False
      FontItalic      =   0   'False
      FontName        =   "MS Sans Serif"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   285
      Left            =   5160
      MaxLength       =   2
      TabIndex        =   9
      Text            =   "Text3"
      Top             =   960
      Width           =   495
   End
   Begin TextBox City 
      FontBold        =   0   'False
      FontItalic      =   0   'False
      FontName        =   "MS Sans Serif"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   285
      Left            =   3240
      MaxLength       =   15
      TabIndex        =   8
      Text            =   "Text2"
      Top             =   960
      Width           =   1815
   End
   Begin TextBox Address 
      FontBold        =   0   'False
      FontItalic      =   0   'False
      FontName        =   "MS Sans Serif"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   285
      Left            =   3240
      MaxLength       =   25
      TabIndex        =   7
      Text            =   "Text1"
      Top             =   600
      Width           =   3255
   End
   Begin MaskEdBox MaskedEdit8 
   End
   Begin MaskEdBox MaskedEdit2 
   End
   Begin MaskEdBox MaskedEdit7 
   End
   Begin MaskEdBox MaskedEdit1 
   End
   Begin CommandButton Command2 
      Cancel          =   -1  'True
      Caption         =   "&Cancel"
      Height          =   500
      Left            =   4800
      TabIndex        =   12
      Top             =   2520
      Width           =   975
   End
   Begin CommandButton Command1 
      Caption         =   "&OK"
      Default         =   -1  'True
      Height          =   500
      Left            =   3240
      TabIndex        =   11
      Top             =   2520
      Width           =   975
   End
   Begin Label lblPhoneListEntry 
      BackStyle       =   0   'Transparent
      Caption         =   "Payphone Version: (left over)"
      Height          =   255
      Index           =   4
      Left            =   3120
      TabIndex        =   14
      Top             =   1560
      Visible         =   0   'False
      Width           =   2775
   End
   Begin Label lblPhoneListEntry 
      BackStyle       =   0   'Transparent
      Caption         =   "Location Address:"
      Height          =   255
      Index           =   3
      Left            =   3120
      TabIndex        =   13
      Top             =   240
      Width           =   2295
   End
   Begin Label Label2 
      Alignment       =   2   'Center
      BorderStyle     =   1   'Fixed Single
      FontBold        =   0   'False
      FontItalic      =   0   'False
      FontName        =   "MS Sans Serif"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   225
      Left            =   3240
      TabIndex        =   15
      Top             =   1920
      Visible         =   0   'False
      Width           =   855
   End
   Begin Label lblPhoneListEntry 
      BackStyle       =   0   'Transparent
      Caption         =   "Payphone PIC:"
      Height          =   255
      Index           =   2
      Left            =   360
      TabIndex        =   5
      Top             =   2160
      Width           =   2295
   End
   Begin Label lblPhoneListEntry 
      BackStyle       =   0   'Transparent
      Caption         =   "Payphone Type && ID:"
      Height          =   255
      Index           =   1
      Left            =   360
      TabIndex        =   2
      Top             =   1200
      Width           =   2295
   End
   Begin Label lblPhoneListEntry 
      BackStyle       =   0   'Transparent
      Caption         =   "Payphone ANI:"
      Height          =   255
      Index           =   0
      Left            =   360
      TabIndex        =   0
      Top             =   240
      Width           =   2295
   End
End

