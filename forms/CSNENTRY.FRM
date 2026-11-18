VERSION 2.00
Begin Form frmCSNEntry 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   Caption         =   "CSN Entry"
   ClientHeight    =   4485
   ClientLeft      =   1125
   ClientTop       =   1905
   ClientWidth     =   5070
   Height          =   4995
   Left            =   1065
   LinkTopic       =   "Form1"
   MaxButton       =   0   'False
   ScaleHeight     =   4485
   ScaleWidth      =   5070
   Top             =   1455
   Width           =   5190
   Begin CommandButton cmdCancel 
      Cancel          =   -1  'True
      Caption         =   "&Cancel"
      Height          =   500
      Left            =   1200
      TabIndex        =   13
      Top             =   3720
      Width           =   975
   End
   Begin CommandButton cmdOk 
      Caption         =   "&OK"
      Default         =   -1  'True
      Height          =   500
      Left            =   3000
      TabIndex        =   12
      Top             =   3720
      Width           =   975
   End
   Begin MaskEdBox meSecondPeriod 
   End
   Begin MaskEdBox meInitialPeriod 
   End
   Begin MaskEdBox meSecondAmount 
   End
   Begin MaskEdBox meInitialAmount 
   End
   Begin ComboBox cboRateType 
      Height          =   315
      Left            =   1800
      Style           =   2   'Dropdown List
      TabIndex        =   7
      Top             =   1140
      Width           =   1815
   End
   Begin MaskEdBox mePhoneNumber 
   End
   Begin Label lblSecondPeriod 
      Alignment       =   2   'Center
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      Caption         =   "Second Period"
      Height          =   195
      Left            =   2970
      TabIndex        =   5
      Top             =   2460
      Width           =   1275
   End
   Begin Label lblInitialPeriod 
      Alignment       =   2   'Center
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      Caption         =   "Initial Period"
      Height          =   195
      Left            =   3120
      TabIndex        =   4
      Top             =   1680
      Width           =   1095
   End
   Begin Label lblSecondAmount 
      Alignment       =   2   'Center
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      Caption         =   "Second Amount"
      Height          =   195
      Left            =   825
      TabIndex        =   3
      Top             =   2460
      Width           =   1365
   End
   Begin Label lblInitialAmount 
      Alignment       =   2   'Center
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      Caption         =   "Initial Amount"
      Height          =   195
      Left            =   915
      TabIndex        =   2
      Top             =   1680
      Width           =   1185
   End
   Begin Label lblRateType 
      Alignment       =   2   'Center
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      Caption         =   "Rate Type"
      Height          =   195
      Left            =   2310
      TabIndex        =   1
      Top             =   900
      Width           =   915
   End
   Begin Label lblPhoneNumber 
      AutoSize        =   -1  'True
      BackColor       =   &H00C0C0C0&
      Caption         =   "Phone Number"
      Height          =   195
      Left            =   2100
      TabIndex        =   0
      Top             =   180
      Width           =   1260
   End
End

