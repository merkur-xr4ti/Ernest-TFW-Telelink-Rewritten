VERSION 2.00
Begin Form ScrollIt 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   Caption         =   "Scrolling Message"
   ClientHeight    =   2325
   ClientLeft      =   1485
   ClientTop       =   2400
   ClientWidth     =   6960
   FontBold        =   0   'False
   FontItalic      =   0   'False
   FontName        =   "MS Sans Serif"
   FontSize        =   8.25
   FontStrikethru  =   0   'False
   FontUnderline   =   0   'False
   Height          =   2835
   Icon            =   SCROLLIT.FRX:0000
   Left            =   1425
   LinkTopic       =   "Form6"
   MaxButton       =   0   'False
   ScaleHeight     =   2325
   ScaleWidth      =   6960
   Top             =   1950
   Width           =   7080
   Begin Gauge Gauge1 
   End
   Begin CommandButton cmdTest 
      Caption         =   "&Test"
      Height          =   495
      Left            =   2880
      TabIndex        =   0
      Top             =   1680
      Width           =   1215
   End
   Begin TextBox txtMessage 
      BackColor       =   &H00FFFFFF&
      FontBold        =   0   'False
      FontItalic      =   0   'False
      FontName        =   "Terminal"
      FontSize        =   9
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   615
      Left            =   240
      MaxLength       =   96
      MultiLine       =   -1  'True
      ScrollBars      =   1
      TabIndex        =   1
      Text            =   "Replace this string with one of your own."
      Top             =   840
      Width           =   6495
   End
   Begin UnknownControl_0x16 shpTestBox 
   End
   Begin Label lblDate_Time 
      FontBold        =   0   'False
      FontItalic      =   0   'False
      FontName        =   "Terminal"
      FontSize        =   9
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   255
      Left            =   2280
      TabIndex        =   5
      Top             =   1080
      Visible         =   0   'False
      Width           =   2415
   End
   Begin Label lblTestMessage 
      Caption         =   "12345678901234567890"
      FontBold        =   0   'False
      FontItalic      =   0   'False
      FontName        =   "Terminal"
      FontSize        =   9
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   255
      Left            =   2280
      TabIndex        =   4
      Top             =   840
      Visible         =   0   'False
      Width           =   2415
   End
   Begin Label lblInstructions 
      Alignment       =   2   'Center
      BackColor       =   &H00C0C0C0&
      Caption         =   "Enter a message to be scrolled on your LCD display.  You may read/enter up to 96 characters."
      ForeColor       =   &H00000000&
      Height          =   555
      Left            =   240
      TabIndex        =   2
      Top             =   120
      Width           =   6495
   End
End

