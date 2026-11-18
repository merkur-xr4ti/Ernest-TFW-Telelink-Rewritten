VERSION 2.00
Begin Form frmConnect 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   Caption         =   "Connect to a Phone"
   ClientHeight    =   2880
   ClientLeft      =   3555
   ClientTop       =   1560
   ClientWidth     =   5895
   Height          =   3390
   Left            =   3495
   LinkTopic       =   "Form1"
   MaxButton       =   0   'False
   ScaleHeight     =   2880
   ScaleWidth      =   5895
   Top             =   1110
   Width           =   6015
   Begin Frame fraUSA 
      BackColor       =   &H00C0C0C0&
      Caption         =   " Phone To Call "
      Height          =   2475
      Left            =   120
      TabIndex        =   21
      Top             =   120
      Width           =   3075
      Begin MaskEdBox mePIC 
      End
      Begin MaskEdBox meOnePlus 
      End
      Begin MaskEdBox meAreaCode 
      End
      Begin MaskEdBox mePhoneNumber 
      End
      Begin MaskEdBox meID 
      End
      Begin Label Label1 
         Alignment       =   1   'Right Justify
         BackColor       =   &H00C0C0C0&
         Caption         =   "PIC"
         Height          =   255
         Left            =   120
         TabIndex        =   11
         Top             =   345
         Width           =   1335
      End
      Begin Label Label2 
         Alignment       =   1   'Right Justify
         BackColor       =   &H00C0C0C0&
         Caption         =   "1+"
         Height          =   255
         Left            =   180
         TabIndex        =   12
         Top             =   720
         Width           =   1335
      End
      Begin Label Label3 
         Alignment       =   1   'Right Justify
         BackColor       =   &H00C0C0C0&
         Caption         =   "Area Code"
         Height          =   255
         Left            =   180
         TabIndex        =   13
         Top             =   1140
         Width           =   1335
      End
      Begin Label Label4 
         Alignment       =   1   'Right Justify
         BackColor       =   &H00C0C0C0&
         Caption         =   "Phone Number"
         Height          =   255
         Left            =   180
         TabIndex        =   14
         Top             =   1560
         Width           =   1335
      End
      Begin Label Label5 
         Alignment       =   1   'Right Justify
         BackColor       =   &H00C0C0C0&
         Caption         =   "ID Number"
         Height          =   255
         Left            =   180
         TabIndex        =   15
         Top             =   1980
         Width           =   1335
      End
   End
   Begin Frame fraCallStatus 
      BackColor       =   &H00C0C0C0&
      Height          =   2540
      Left            =   120
      TabIndex        =   16
      Top             =   240
      Visible         =   0   'False
      Width           =   3135
      Begin Label lblTimeRemaining 
         Alignment       =   2   'Center
         BackColor       =   &H00C0C0C0&
         Height          =   260
         Left            =   480
         TabIndex        =   20
         Top             =   1920
         Width           =   2175
      End
      Begin Label Label8 
         Alignment       =   2   'Center
         AutoSize        =   -1  'True
         BackColor       =   &H00C0C0C0&
         Caption         =   "Time Remaining"
         Height          =   195
         Left            =   870
         TabIndex        =   19
         Top             =   1680
         Width           =   1395
      End
      Begin Label lblNumberCalled 
         Alignment       =   2   'Center
         BackColor       =   &H00C0C0C0&
         Height          =   255
         Left            =   450
         TabIndex        =   18
         Top             =   675
         Width           =   2175
      End
      Begin Label lblStatus 
         Alignment       =   2   'Center
         AutoSize        =   -1  'True
         BackColor       =   &H00C0C0C0&
         Caption         =   "Waiting for connection."
         Height          =   195
         Left            =   555
         TabIndex        =   17
         Top             =   360
         Width           =   2025
      End
   End
   Begin CommandButton cmdLookup 
      Caption         =   "&Lookup"
      Height          =   495
      Left            =   4560
      TabIndex        =   25
      Top             =   240
      Width           =   975
   End
   Begin Timer tmrBatch 
      Enabled         =   0   'False
      Interval        =   1000
      Left            =   5280
      Top             =   0
   End
   Begin OptionButton optD1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Phone Type D1 / Data"
      Height          =   375
      Left            =   3480
      TabIndex        =   8
      Top             =   2040
      Width           =   2295
   End
   Begin OptionButton optD3 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Phone Type D3 / ETX"
      Height          =   375
      Left            =   3480
      TabIndex        =   7
      Top             =   1680
      Width           =   2295
   End
   Begin CommandButton cmdSkip 
      Caption         =   "&Skip"
      Height          =   500
      Left            =   4560
      TabIndex        =   6
      Top             =   960
      Visible         =   0   'False
      Width           =   975
   End
   Begin CommandButton cmdCancel 
      Cancel          =   -1  'True
      Caption         =   "&Cancel"
      Height          =   500
      Left            =   3480
      TabIndex        =   5
      Top             =   960
      Width           =   975
   End
   Begin CommandButton cmdOk 
      Caption         =   "&OK"
      Default         =   -1  'True
      Height          =   500
      Left            =   3480
      TabIndex        =   4
      Top             =   240
      Width           =   975
   End
   Begin Frame fraChina 
      BackColor       =   &H00C0C0C0&
      Caption         =   " Phone To Call "
      Height          =   2475
      Left            =   120
      TabIndex        =   22
      Top             =   240
      Width           =   3075
      Begin MaskEdBox meChinaID 
      End
      Begin MaskEdBox meChinaPhoneNumber 
      End
      Begin Label Label11 
         AutoSize        =   -1  'True
         BackColor       =   &H00C0C0C0&
         Caption         =   "ID Number"
         Height          =   195
         Left            =   480
         TabIndex        =   23
         Top             =   1560
         Width           =   915
      End
      Begin Label Label10 
         AutoSize        =   -1  'True
         BackColor       =   &H00C0C0C0&
         Caption         =   "Phone Number"
         Height          =   195
         Left            =   480
         TabIndex        =   24
         Top             =   720
         Width           =   1260
      End
   End
End

