VERSION 2.00
Begin Form frmCDRViewer 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   1   'Fixed Single
   Caption         =   "CDR Viewer"
   ClientHeight    =   5415
   ClientLeft      =   105
   ClientTop       =   1125
   ClientWidth     =   9585
   Height          =   5895
   Icon            =   FRMCDRVI.FRX:0000
   Left            =   90
   LinkTopic       =   "Form1"
   ScaleHeight     =   5415
   ScaleMode       =   3
   ScaleWidth      =   9585
   Top             =   675
   Width           =   9615
   Begin SSFrame fraCDRv 
      Begin SSCheck chkHideRepeats 
      End
      Begin ListBox lstPL 
         FontBold        =   0   'False
         FontItalic      =   0   'False
         FontName        =   "MS Sans Serif"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   810
         Left            =   7920
         TabIndex        =   6
         Top             =   360
         Width           =   1575
      End
      Begin Label lblCallAmounts 
         BackStyle       =   0   'Transparent
         Caption         =   "call amounts"
         Height          =   255
         Left            =   120
         TabIndex        =   9
         Top             =   600
         Width           =   5535
      End
      Begin Label lblInfo 
         Alignment       =   2   'Center
         BackStyle       =   0   'Transparent
         BorderStyle     =   1   'Fixed Single
         Caption         =   "Phone lists:"
         Height          =   255
         Left            =   7920
         TabIndex        =   10
         Top             =   120
         Width           =   1140
         WordWrap        =   -1  'True
      End
      Begin Label lblDialAround 
         BackStyle       =   0   'Transparent
         Caption         =   "dial around"
         Height          =   255
         Left            =   120
         TabIndex        =   8
         Top             =   840
         Width           =   5775
      End
      Begin Label lblDateRange 
         BackStyle       =   0   'Transparent
         Caption         =   "date range"
         Height          =   255
         Left            =   120
         TabIndex        =   7
         Top             =   360
         Width           =   5415
      End
      Begin Label lblFileName 
         BackStyle       =   0   'Transparent
         Caption         =   "filename"
         ForeColor       =   &H00FF0000&
         Height          =   255
         Left            =   120
         TabIndex        =   5
         Top             =   120
         Width           =   5655
      End
   End
   Begin ListBox lstCallDetail 
      FontBold        =   0   'False
      FontItalic      =   0   'False
      FontName        =   "Courier New"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   3180
      Left            =   1500
      TabIndex        =   1
      Top             =   1680
      Width           =   8055
   End
   Begin ListBox lstCallTypes 
      FontBold        =   0   'False
      FontItalic      =   0   'False
      FontName        =   "Courier New"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   3180
      Left            =   30
      TabIndex        =   0
      Top             =   1680
      Width           =   1455
   End
   Begin Label lblInfo2 
      Alignment       =   2   'Center
      BackStyle       =   0   'Transparent
      Caption         =   "Valid calls with NO Coins Collected are marked with an * at the end of the record"
      ForeColor       =   &H00FF0000&
      Height          =   255
      Left            =   600
      TabIndex        =   12
      Top             =   4920
      Width           =   8415
   End
   Begin Label lblDetails 
      BackStyle       =   0   'Transparent
      Caption         =   "ANI                 Date     Time  Phone Called        Len    Amount   *"
      FontBold        =   -1  'True
      FontItalic      =   0   'False
      FontName        =   "Courier New"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   255
      Left            =   1560
      TabIndex        =   4
      Top             =   1440
      Width           =   7770
   End
   Begin Label lblCallTypes 
      BackStyle       =   0   'Transparent
      Caption         =   "Call Types"
      FontBold        =   -1  'True
      FontItalic      =   0   'False
      FontName        =   "Courier New"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   255
      Left            =   60
      TabIndex        =   3
      Top             =   1440
      Width           =   1395
   End
End
