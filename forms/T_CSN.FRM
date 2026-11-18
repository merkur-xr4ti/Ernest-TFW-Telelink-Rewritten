VERSION 2.00
Begin Form frmCSNTable 
   BackColor       =   &H00C0C0C0&
   BorderStyle     =   3   'Fixed Double
   Caption         =   "CSN Table"
   ClientHeight    =   5085
   ClientLeft      =   1245
   ClientTop       =   1725
   ClientWidth     =   7515
   Height          =   5595
   Left            =   1185
   LinkTopic       =   "Form4"
   MaxButton       =   0   'False
   ScaleHeight     =   5085
   ScaleWidth      =   7515
   Top             =   1275
   Width           =   7635
   Begin ListBox List4 
      Height          =   420
      Left            =   6360
      TabIndex        =   19
      Top             =   4680
      Visible         =   0   'False
      Width           =   1215
   End
   Begin Frame frmExportCSNInfo 
      BackColor       =   &H0000FFFF&
      Caption         =   "CSN Table "
      Height          =   3615
      Left            =   600
      TabIndex        =   14
      Top             =   600
      Width           =   6255
      Begin CommandButton Command3 
         Caption         =   "&3 - Cancel this operation."
         Height          =   495
         Left            =   120
         TabIndex        =   18
         Top             =   2760
         Width           =   6015
      End
      Begin CommandButton Command2 
         Caption         =   "&2 - Save the current CSN with a different name and continue."
         Height          =   495
         Left            =   120
         TabIndex        =   17
         Top             =   2160
         Width           =   6015
      End
      Begin CommandButton Command1 
         Caption         =   "&1 - Save the current CSN with it's current name and continue."
         Height          =   495
         Left            =   120
         TabIndex        =   16
         Top             =   1560
         Width           =   6015
      End
      Begin TextBox txtCSNWarningExplanation 
         Height          =   855
         Left            =   120
         MultiLine       =   -1  'True
         TabIndex        =   15
         Text            =   "     Before a comma delimitted file can be produced the current file must be saved.  You may choose to save the file with it's current name, to save the file with a different name, or to cancel this operation."
         Top             =   480
         Width           =   6015
      End
   End
   Begin ListBox List3 
      Height          =   225
      Left            =   4920
      TabIndex        =   13
      Top             =   4920
      Visible         =   0   'False
      Width           =   975
   End
   Begin CommandButton cmdEdit 
      Caption         =   "&Edit"
      Enabled         =   0   'False
      Height          =   500
      Left            =   2640
      TabIndex        =   9
      Top             =   4440
      Width           =   975
   End
   Begin CommandButton cmdAdd 
      Caption         =   "&Add"
      Height          =   500
      Left            =   1440
      TabIndex        =   8
      Top             =   4440
      Width           =   975
   End
   Begin ListBox List2 
      Height          =   225
      Left            =   3480
      Sorted          =   -1  'True
      TabIndex        =   2
      Top             =   4920
      Visible         =   0   'False
      Width           =   975
   End
   Begin ListBox List1 
      FontBold        =   0   'False
      FontItalic      =   0   'False
      FontName        =   "Courier New"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   3180
      Left            =   120
      TabIndex        =   4
      Top             =   480
      Width           =   7155
   End
   Begin CommandButton cmdClear 
      Caption         =   "&Clear"
      Enabled         =   0   'False
      Height          =   495
      Left            =   5040
      TabIndex        =   5
      Top             =   4440
      Width           =   1215
   End
   Begin CommandButton cmdDelete 
      Caption         =   "&Delete"
      Enabled         =   0   'False
      Height          =   500
      Left            =   3840
      TabIndex        =   7
      Top             =   4440
      Width           =   975
   End
   Begin Label lblChange 
      Alignment       =   2   'Center
      BackStyle       =   0   'Transparent
      Caption         =   "Don't forget to save your changes!"
      ForeColor       =   &H000000FF&
      Height          =   255
      Left            =   120
      TabIndex        =   12
      Tag             =   "No"
      Top             =   4200
      Visible         =   0   'False
      Width           =   7095
   End
   Begin Label lblCurrFile 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Working File:"
      Height          =   255
      Left            =   180
      TabIndex        =   11
      Top             =   3840
      Width           =   1215
   End
   Begin Label lblWorkingFile 
      BackColor       =   &H00C0C0C0&
      Height          =   255
      Left            =   1440
      TabIndex        =   10
      Top             =   3840
      Width           =   5775
   End
   Begin Label Label7 
      BackColor       =   &H00C0C0C0&
      Caption         =   "2nd $/Time"
      Height          =   255
      Left            =   5520
      TabIndex        =   6
      Top             =   240
      Width           =   1095
   End
   Begin Label Label4 
      BackColor       =   &H00C0C0C0&
      Caption         =   "1st $/Time"
      Height          =   255
      Left            =   3960
      TabIndex        =   3
      Top             =   240
      Width           =   975
   End
   Begin Label Label2 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Rate Type"
      Height          =   255
      Left            =   2280
      TabIndex        =   1
      Top             =   240
      Width           =   975
   End
   Begin Label Label1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Phone Number"
      Height          =   252
      Left            =   180
      TabIndex        =   0
      Top             =   240
      Width           =   1392
   End
End

