VERSION 2.00
Begin Form frmBatchSetupChina 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   Caption         =   "Batch File Setup"
   ClientHeight    =   4110
   ClientLeft      =   1935
   ClientTop       =   1440
   ClientWidth     =   6270
   Height          =   4620
   Left            =   1875
   LinkTopic       =   "Form1"
   MaxButton       =   0   'False
   ScaleHeight     =   4110
   ScaleWidth      =   6270
   Top             =   990
   Width           =   6390
   Begin CheckBox Check1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Download Time && Date"
      Height          =   264
      Index           =   9
      Left            =   360
      TabIndex        =   27
      Top             =   2280
      Width           =   3132
   End
   Begin OptionButton Option1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Active Phone List"
      Height          =   260
      Left            =   360
      TabIndex        =   20
      Top             =   2640
      Width           =   3135
   End
   Begin TextBox Text1 
      Height          =   300
      Index           =   10
      Left            =   3720
      TabIndex        =   21
      Top             =   2640
      Width           =   2295
   End
   Begin CheckBox Check2 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Clear SMDR"
      Height          =   264
      Index           =   1
      Left            =   3720
      TabIndex        =   5
      Top             =   840
      Width           =   2172
   End
   Begin CheckBox Check2 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Clear Counters"
      Height          =   264
      Index           =   0
      Left            =   3720
      TabIndex        =   3
      Top             =   480
      Width           =   2172
   End
   Begin TextBox Text1 
      Height          =   300
      Index           =   8
      Left            =   3720
      TabIndex        =   19
      Top             =   3360
      Width           =   2295
   End
   Begin TextBox Text1 
      Height          =   300
      Index           =   7
      Left            =   3720
      TabIndex        =   17
      Top             =   1920
      Width           =   2295
   End
   Begin TextBox Text1 
      Height          =   300
      Index           =   6
      Left            =   3720
      TabIndex        =   15
      Top             =   1560
      Width           =   2295
   End
   Begin TextBox Text1 
      Height          =   300
      Index           =   5
      Left            =   3720
      TabIndex        =   13
      Top             =   1200
      Width           =   2295
   End
   Begin TextBox Text1 
      Height          =   300
      Index           =   4
      Left            =   3720
      TabIndex        =   11
      Top             =   1920
      Width           =   2295
   End
   Begin TextBox Text1 
      Height          =   300
      Index           =   3
      Left            =   3720
      TabIndex        =   9
      Top             =   1560
      Width           =   2295
   End
   Begin TextBox Text1 
      Height          =   300
      Index           =   2
      Left            =   3720
      TabIndex        =   7
      Top             =   1200
      Width           =   2295
   End
   Begin CheckBox Check1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Download Speed Dial Numbers"
      Height          =   264
      Index           =   8
      Left            =   360
      TabIndex        =   18
      Top             =   3480
      Width           =   3132
   End
   Begin CheckBox Check1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Download Rate Table"
      Height          =   264
      Index           =   7
      Left            =   360
      TabIndex        =   16
      Top             =   1920
      Width           =   3132
   End
   Begin CheckBox Check1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Download Programmable Features"
      Height          =   264
      Index           =   6
      Left            =   360
      TabIndex        =   14
      Top             =   1560
      Width           =   3132
   End
   Begin CheckBox Check1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Download CSN Table"
      Height          =   264
      Index           =   5
      Left            =   360
      TabIndex        =   12
      Top             =   1200
      Width           =   3132
   End
   Begin CheckBox Check1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Download AOS Script #3"
      Height          =   264
      Index           =   4
      Left            =   360
      TabIndex        =   10
      Top             =   1920
      Width           =   3132
   End
   Begin CheckBox Check1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Download AOS Script #2"
      Height          =   264
      Index           =   3
      Left            =   360
      TabIndex        =   8
      Top             =   1560
      Width           =   3132
   End
   Begin CheckBox Check1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Download AOS Script #1"
      Height          =   264
      Index           =   2
      Left            =   360
      TabIndex        =   6
      Top             =   1200
      Width           =   3132
   End
   Begin CheckBox Check1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Upload SMDR Info"
      Height          =   264
      Index           =   1
      Left            =   360
      TabIndex        =   4
      Top             =   840
      Width           =   3132
   End
   Begin CheckBox Check1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Upload Collection Data"
      Height          =   264
      Index           =   0
      Left            =   360
      TabIndex        =   0
      Top             =   480
      Width           =   3132
   End
   Begin CheckBox Check1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Download Scrolling Message"
      Height          =   264
      Index           =   12
      Left            =   360
      TabIndex        =   25
      Top             =   3120
      Visible         =   0   'False
      Width           =   3132
   End
   Begin TextBox Text1 
      Height          =   300
      Index           =   12
      Left            =   3720
      TabIndex        =   26
      Top             =   3720
      Visible         =   0   'False
      Width           =   2295
   End
   Begin Label lblChange 
      Alignment       =   2   'Center
      BackStyle       =   0   'Transparent
      Caption         =   "Don't forget to save your changes!"
      ForeColor       =   &H000000FF&
      Height          =   255
      Left            =   0
      TabIndex        =   24
      Tag             =   "No"
      Top             =   3720
      Visible         =   0   'False
      Width           =   6255
   End
   Begin Label Label4 
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      Caption         =   "Working File:"
      Height          =   195
      Left            =   360
      TabIndex        =   23
      Top             =   3120
      Width           =   1140
   End
   Begin Label Label1 
      BackColor       =   &H00C0C0C0&
      Height          =   255
      Left            =   1920
      TabIndex        =   22
      Top             =   3120
      Width           =   4095
   End
   Begin Label Label3 
      Alignment       =   2   'Center
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      Caption         =   "Control Files"
      FontBold        =   -1  'True
      FontItalic      =   0   'False
      FontName        =   "MS Sans Serif"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   -1  'True
      Height          =   195
      Left            =   4260
      TabIndex        =   2
      Top             =   120
      Width           =   1095
   End
   Begin Label Label2 
      Alignment       =   2   'Center
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      Caption         =   "Batch Operation"
      FontBold        =   -1  'True
      FontItalic      =   0   'False
      FontName        =   "MS Sans Serif"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   -1  'True
      Height          =   195
      Left            =   1215
      TabIndex        =   1
      Top             =   120
      Width           =   1425
   End
End

