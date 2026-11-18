VERSION 2.00
Begin Form frmCollectionStatus 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   Caption         =   "Collection Status"
   ClientHeight    =   6165
   ClientLeft      =   1725
   ClientTop       =   1845
   ClientWidth     =   7800
   Height          =   6675
   Left            =   1665
   LinkTopic       =   "Form3"
   MaxButton       =   0   'False
   ScaleHeight     =   6165
   ScaleWidth      =   7800
   Top             =   1395
   Width           =   7920
   Begin CommandButton cmdCalculate 
      Caption         =   "&Last Collected"
      Height          =   495
      Left            =   840
      TabIndex        =   66
      Top             =   5520
      Width           =   1695
   End
   Begin ListBox lstCFWTempFiles2 
      Height          =   225
      Left            =   6000
      TabIndex        =   65
      Top             =   5520
      Visible         =   0   'False
      Width           =   1695
   End
   Begin ListBox lstCFWTempFiles 
      Enabled         =   0   'False
      Height          =   225
      Left            =   6000
      Sorted          =   -1  'True
      TabIndex        =   64
      Top             =   5160
      Visible         =   0   'False
      Width           =   1695
   End
   Begin FileListBox flbCFW 
      Height          =   225
      Left            =   6000
      Pattern         =   "*.cfw"
      TabIndex        =   58
      Top             =   5880
      Visible         =   0   'False
      Width           =   1695
   End
   Begin Frame Frame1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Last Collected"
      Height          =   615
      Left            =   120
      TabIndex        =   59
      Top             =   840
      Width           =   7215
      Begin TextBox txtAmountCollected 
         Height          =   285
         Left            =   5760
         TabIndex        =   60
         Top             =   195
         Width           =   1215
      End
      Begin TextBox txtDateCollected 
         Height          =   285
         Left            =   2160
         TabIndex        =   63
         Top             =   195
         Width           =   1335
      End
      Begin Label Label17 
         Alignment       =   1   'Right Justify
         AutoSize        =   -1  'True
         BackStyle       =   0   'Transparent
         Caption         =   "Amount Collected"
         Height          =   195
         Left            =   4080
         TabIndex        =   62
         Top             =   240
         Width           =   1500
      End
      Begin Label Label16 
         Alignment       =   1   'Right Justify
         AutoSize        =   -1  'True
         BackStyle       =   0   'Transparent
         Caption         =   "Date Collected"
         Height          =   195
         Left            =   720
         TabIndex        =   61
         Top             =   240
         Width           =   1275
      End
   End
   Begin MaskEdBox meIDBox 
   End
   Begin TextBox txtPhoneBox 
      Height          =   285
      Left            =   1080
      TabIndex        =   19
      Top             =   120
      Width           =   1695
   End
   Begin CommandButton cmdIDSearch 
      Caption         =   "Search"
      Height          =   255
      Left            =   6360
      TabIndex        =   56
      Top             =   120
      Width           =   855
   End
   Begin CommandButton cmdPhoneSearch 
      Caption         =   "Search"
      Height          =   255
      Left            =   2880
      TabIndex        =   55
      Top             =   120
      Width           =   855
   End
   Begin SSFrame Frame3D9 
      Begin MaskEdBox MaskedEdit18 
      End
      Begin MaskEdBox MaskedEdit17 
      End
      Begin Label Label24 
         BackStyle       =   0   'Transparent
         Caption         =   "Calls"
         Height          =   260
         Left            =   120
         TabIndex        =   9
         Top             =   360
         Width           =   975
      End
      Begin Label Label23 
         BackStyle       =   0   'Transparent
         Caption         =   "Amount"
         Height          =   260
         Left            =   120
         TabIndex        =   10
         Top             =   720
         Width           =   975
      End
   End
   Begin SSFrame Frame3D8 
      Begin MaskEdBox MaskedEdit16 
      End
      Begin MaskEdBox MaskedEdit15 
      End
      Begin Label Label20 
         BackStyle       =   0   'Transparent
         Caption         =   "Calls"
         Height          =   260
         Left            =   120
         TabIndex        =   14
         Top             =   360
         Width           =   975
      End
      Begin Label Label19 
         BackStyle       =   0   'Transparent
         Caption         =   "Amount"
         Height          =   260
         Left            =   120
         TabIndex        =   15
         Top             =   720
         Width           =   975
      End
   End
   Begin SSFrame Frame3D7 
      Begin MaskEdBox MaskedEdit14 
      End
      Begin MaskEdBox MaskedEdit13 
      End
      Begin Label Label15 
         BackStyle       =   0   'Transparent
         Caption         =   "Calls"
         Height          =   260
         Left            =   120
         TabIndex        =   21
         Top             =   360
         Width           =   975
      End
      Begin Label Label14 
         BackStyle       =   0   'Transparent
         Caption         =   "Amount"
         Height          =   260
         Left            =   120
         TabIndex        =   22
         Top             =   720
         Width           =   975
      End
   End
   Begin SSFrame Frame3D6 
      Begin MaskEdBox MaskedEdit11 
      End
      Begin MaskEdBox MaskedEdit12 
      End
      Begin Label Label13 
         BackStyle       =   0   'Transparent
         Caption         =   "Amount"
         Height          =   260
         Left            =   120
         TabIndex        =   46
         Top             =   720
         Width           =   975
      End
      Begin Label Label9 
         BackStyle       =   0   'Transparent
         Caption         =   "Calls"
         Height          =   260
         Left            =   120
         TabIndex        =   47
         Top             =   360
         Width           =   975
      End
   End
   Begin SSFrame Frame3D5 
      Begin MaskEdBox MaskedEdit9 
      End
      Begin MaskEdBox MaskedEdit10 
      End
      Begin Label Label10 
         BackStyle       =   0   'Transparent
         Caption         =   "Amount"
         Height          =   260
         Left            =   120
         TabIndex        =   48
         Top             =   720
         Width           =   975
      End
      Begin Label Label12 
         BackStyle       =   0   'Transparent
         Caption         =   "Calls"
         Height          =   255
         Left            =   120
         TabIndex        =   49
         Top             =   360
         Width           =   975
      End
   End
   Begin SSFrame Frame3D4 
      Begin MaskEdBox MaskedEdit7 
      End
      Begin MaskEdBox MaskedEdit8 
      End
      Begin Label Label11 
         BackStyle       =   0   'Transparent
         Caption         =   "Amount"
         Height          =   255
         Left            =   120
         TabIndex        =   45
         Top             =   720
         Width           =   975
      End
      Begin Label Label8 
         BackStyle       =   0   'Transparent
         Caption         =   "Calls"
         Height          =   260
         Left            =   120
         TabIndex        =   44
         Top             =   360
         Width           =   975
      End
   End
   Begin SSFrame Frame3D3 
      Begin MaskEdBox MaskedEdit6 
      End
      Begin MaskEdBox MaskedEdit5 
      End
      Begin Label Label7 
         BackStyle       =   0   'Transparent
         Caption         =   "Calls"
         Height          =   260
         Left            =   120
         TabIndex        =   40
         Top             =   360
         Width           =   975
      End
      Begin Label Label6 
         BackStyle       =   0   'Transparent
         Caption         =   "Amount"
         Height          =   260
         Left            =   120
         TabIndex        =   39
         Top             =   720
         Width           =   975
      End
   End
   Begin SSFrame Frame3D2 
      Begin MaskEdBox MaskedEdit3 
      End
      Begin MaskEdBox MaskedEdit4 
      End
      Begin Label Label5 
         BackStyle       =   0   'Transparent
         Caption         =   "Amount"
         Height          =   260
         Left            =   120
         TabIndex        =   34
         Top             =   720
         Width           =   975
      End
      Begin Label Label4 
         BackStyle       =   0   'Transparent
         Caption         =   "Calls"
         Height          =   260
         Left            =   120
         TabIndex        =   33
         Top             =   360
         Width           =   975
      End
   End
   Begin SSFrame Frame3D1 
      Begin MaskEdBox MaskedEdit1 
      End
      Begin MaskEdBox MaskedEdit2 
      End
      Begin Label Label21 
         BackStyle       =   0   'Transparent
         Caption         =   "Coin Box"
         Height          =   260
         Left            =   120
         TabIndex        =   28
         Top             =   360
         Width           =   975
      End
      Begin Label Label22 
         BackStyle       =   0   'Transparent
         Caption         =   "Cumulative"
         Height          =   260
         Left            =   120
         TabIndex        =   27
         Top             =   720
         Width           =   975
      End
   End
   Begin CommandButton cmdPrevious 
      Caption         =   "&Previous"
      Enabled         =   0   'False
      Height          =   500
      Left            =   4200
      TabIndex        =   5
      Top             =   5520
      Width           =   975
   End
   Begin CommandButton cmdNext 
      Caption         =   "&Next"
      Height          =   500
      Left            =   5520
      TabIndex        =   4
      Top             =   5520
      Width           =   975
   End
   Begin CommandButton cmdClear 
      Caption         =   "&Clear"
      Height          =   495
      Left            =   2880
      TabIndex        =   3
      Top             =   5520
      Width           =   975
   End
   Begin Label lblCurrFile 
      Alignment       =   1   'Right Justify
      BackColor       =   &H00C0C0C0&
      Caption         =   "Working File:"
      Height          =   255
      Left            =   120
      TabIndex        =   54
      Top             =   5160
      Width           =   1215
   End
   Begin Label Label25 
      BackColor       =   &H00C0C0C0&
      Height          =   255
      Left            =   1380
      TabIndex        =   53
      Top             =   5160
      Width           =   2895
   End
   Begin Label Label18 
      Alignment       =   2   'Center
      BorderStyle     =   1   'Fixed Single
      Height          =   255
      Left            =   1080
      TabIndex        =   18
      Top             =   480
      Width           =   1215
   End
   Begin Label Label3 
      Alignment       =   1   'Right Justify
      BackColor       =   &H00C0C0C0&
      Caption         =   "Date"
      Height          =   255
      Left            =   360
      TabIndex        =   2
      Top             =   480
      Width           =   615
   End
   Begin Label Label2 
      Alignment       =   1   'Right Justify
      BackColor       =   &H00C0C0C0&
      Caption         =   "ID #"
      Height          =   255
      Left            =   3960
      TabIndex        =   1
      Top             =   120
      Width           =   495
   End
   Begin Label Label1 
      Alignment       =   1   'Right Justify
      BackColor       =   &H00C0C0C0&
      Caption         =   "Phone #"
      Height          =   255
      Left            =   240
      TabIndex        =   0
      Top             =   120
      Width           =   735
   End
End

