VERSION 2.00
Begin Form frmCDRGenerator 
   BackColor       =   &H00C0C0C0&
   Caption         =   "Call Detail Report Generator"
   ClientHeight    =   4905
   ClientLeft      =   735
   ClientTop       =   1455
   ClientWidth     =   8520
   Height          =   4905
   Icon            =   FRMCDRGE.FRX:0000
   Left            =   735
   LinkTopic       =   "Form1"
   ScaleHeight     =   4905
   ScaleWidth      =   8520
   Top             =   1455
   Width           =   8520
   Begin CommandButton cmdViewReport 
      Caption         =   "&View Reports"
      Height          =   615
      Left            =   4440
      TabIndex        =   73
      Top             =   4200
      Width           =   1575
   End
   Begin FileListBox filSMDR 
      Height          =   225
      Left            =   4560
      TabIndex        =   71
      Top             =   6840
      Visible         =   0   'False
      Width           =   1215
   End
   Begin UnknownControl_0x25 Data2 
   End
   Begin TextBox Text2 
      DataField       =   "PIC"
      DataSource      =   "Data2"
      Height          =   285
      Index           =   9
      Left            =   7440
      TabIndex        =   70
      Text            =   "Text2"
      Top             =   7080
      Visible         =   0   'False
      Width           =   1215
   End
   Begin TextBox Text2 
      DataField       =   "StatusCode"
      DataSource      =   "Data2"
      Height          =   285
      Index           =   8
      Left            =   7440
      TabIndex        =   69
      Text            =   "Text2"
      Top             =   6840
      Visible         =   0   'False
      Width           =   1215
   End
   Begin TextBox Text2 
      DataField       =   "Zip"
      DataSource      =   "Data2"
      Height          =   285
      Index           =   7
      Left            =   7440
      TabIndex        =   68
      Text            =   "Text2"
      Top             =   6600
      Visible         =   0   'False
      Width           =   1215
   End
   Begin TextBox Text2 
      DataField       =   "State"
      DataSource      =   "Data2"
      Height          =   285
      Index           =   6
      Left            =   7440
      TabIndex        =   67
      Text            =   "Text2"
      Top             =   6360
      Visible         =   0   'False
      Width           =   1215
   End
   Begin TextBox Text2 
      DataField       =   "City"
      DataSource      =   "Data2"
      Height          =   285
      Index           =   5
      Left            =   7440
      TabIndex        =   66
      Text            =   "Text2"
      Top             =   6120
      Visible         =   0   'False
      Width           =   1215
   End
   Begin TextBox Text2 
      DataField       =   "Address"
      DataSource      =   "Data2"
      Height          =   285
      Index           =   4
      Left            =   6120
      TabIndex        =   65
      Text            =   "Text2"
      Top             =   7080
      Visible         =   0   'False
      Width           =   1215
   End
   Begin TextBox Text2 
      DataField       =   "Version"
      DataSource      =   "Data2"
      Height          =   285
      Index           =   3
      Left            =   6120
      TabIndex        =   64
      Text            =   "Text2"
      Top             =   6840
      Visible         =   0   'False
      Width           =   1215
   End
   Begin TextBox Text2 
      DataField       =   "Type"
      DataSource      =   "Data2"
      Height          =   285
      Index           =   2
      Left            =   6120
      TabIndex        =   63
      Text            =   "Text2"
      Top             =   6600
      Visible         =   0   'False
      Width           =   1215
   End
   Begin TextBox Text2 
      DataField       =   "ID"
      DataSource      =   "Data2"
      Height          =   285
      Index           =   1
      Left            =   6120
      TabIndex        =   62
      Text            =   "Text2"
      Top             =   6360
      Visible         =   0   'False
      Width           =   1215
   End
   Begin TextBox Text2 
      DataField       =   "PhoneNumber"
      DataSource      =   "Data2"
      Height          =   285
      Index           =   0
      Left            =   6120
      TabIndex        =   61
      Text            =   "Text2"
      Top             =   6120
      Visible         =   0   'False
      Width           =   1215
   End
   Begin TextBox Text1 
      DataField       =   "TRANSACTIONCODE"
      DataSource      =   "Data1"
      Height          =   285
      Index           =   16
      Left            =   4320
      TabIndex        =   60
      Text            =   "Text1"
      Top             =   6360
      Visible         =   0   'False
      Width           =   1215
   End
   Begin TextBox Text1 
      DataField       =   "POSTED"
      DataSource      =   "Data1"
      Height          =   285
      Index           =   15
      Left            =   4320
      TabIndex        =   59
      Text            =   "Text1"
      Top             =   6120
      Visible         =   0   'False
      Width           =   1215
   End
   Begin TextBox Text1 
      DataField       =   "TYPEOFCALL"
      DataSource      =   "Data1"
      Height          =   285
      Index           =   14
      Left            =   3000
      TabIndex        =   58
      Text            =   "Text1"
      Top             =   7080
      Visible         =   0   'False
      Width           =   1215
   End
   Begin TextBox Text1 
      DataField       =   "AMOUNT"
      DataSource      =   "Data1"
      Height          =   285
      Index           =   13
      Left            =   3000
      TabIndex        =   57
      Text            =   "Text1"
      Top             =   6840
      Visible         =   0   'False
      Width           =   1215
   End
   Begin TextBox Text1 
      DataField       =   "LENGTHOFCALL"
      DataSource      =   "Data1"
      Height          =   285
      Index           =   12
      Left            =   3000
      TabIndex        =   56
      Text            =   "Text1"
      Top             =   6600
      Visible         =   0   'False
      Width           =   1215
   End
   Begin TextBox Text1 
      DataField       =   "PHONECALLED"
      DataSource      =   "Data1"
      Height          =   285
      Index           =   11
      Left            =   3000
      TabIndex        =   55
      Text            =   "Text1"
      Top             =   6360
      Visible         =   0   'False
      Width           =   1215
   End
   Begin TextBox Text1 
      DataField       =   "TIMEOFCALL"
      DataSource      =   "Data1"
      Height          =   285
      Index           =   10
      Left            =   3000
      TabIndex        =   54
      Text            =   "Text1"
      Top             =   6120
      Visible         =   0   'False
      Width           =   1215
   End
   Begin TextBox Text1 
      DataField       =   "DATEOFCALL"
      DataSource      =   "Data1"
      Height          =   285
      Index           =   9
      Left            =   1560
      TabIndex        =   53
      Text            =   "Text1"
      Top             =   7080
      Visible         =   0   'False
      Width           =   1335
   End
   Begin TextBox Text1 
      DataField       =   "STATUS"
      DataSource      =   "Data1"
      Height          =   285
      Index           =   8
      Left            =   1560
      TabIndex        =   52
      Text            =   "Text1"
      Top             =   6840
      Visible         =   0   'False
      Width           =   1335
   End
   Begin TextBox Text1 
      DataField       =   "PHONETYPE"
      DataSource      =   "Data1"
      Height          =   285
      Index           =   7
      Left            =   1560
      TabIndex        =   51
      Text            =   "Text1"
      Top             =   6600
      Visible         =   0   'False
      Width           =   1335
   End
   Begin TextBox Text1 
      DataField       =   "PIC"
      DataSource      =   "Data1"
      Height          =   285
      Index           =   6
      Left            =   1560
      TabIndex        =   50
      Text            =   "Text1"
      Top             =   6360
      Visible         =   0   'False
      Width           =   1335
   End
   Begin TextBox Text1 
      DataField       =   "ZIP"
      DataSource      =   "Data1"
      Height          =   285
      Index           =   5
      Left            =   1560
      TabIndex        =   49
      Text            =   "Text1"
      Top             =   6120
      Visible         =   0   'False
      Width           =   1335
   End
   Begin TextBox Text1 
      DataField       =   "STATE"
      DataSource      =   "Data1"
      Height          =   285
      Index           =   4
      Left            =   240
      TabIndex        =   48
      Text            =   "Text1"
      Top             =   7080
      Visible         =   0   'False
      Width           =   1215
   End
   Begin TextBox Text1 
      DataField       =   "CITY"
      DataSource      =   "Data1"
      Height          =   285
      Index           =   3
      Left            =   240
      TabIndex        =   47
      Text            =   "Text1"
      Top             =   6840
      Visible         =   0   'False
      Width           =   1215
   End
   Begin TextBox Text1 
      DataField       =   "STREET"
      DataSource      =   "Data1"
      Height          =   285
      Index           =   2
      Left            =   240
      TabIndex        =   46
      Text            =   "Text1"
      Top             =   6600
      Visible         =   0   'False
      Width           =   1215
   End
   Begin TextBox Text1 
      DataField       =   "ID"
      DataSource      =   "Data1"
      Height          =   285
      Index           =   1
      Left            =   240
      TabIndex        =   45
      Text            =   "Text1"
      Top             =   6360
      Visible         =   0   'False
      Width           =   1215
   End
   Begin UnknownControl_0x25 Data1 
   End
   Begin Frame Frame4 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Call Amounts"
      Height          =   1335
      Left            =   6360
      TabIndex        =   43
      Top             =   120
      Width           =   2055
      Begin OptionButton optAllAmounts 
         BackColor       =   &H00C0C0C0&
         Caption         =   "All Amounts"
         Height          =   255
         Left            =   120
         TabIndex        =   2
         Top             =   240
         Width           =   1575
      End
      Begin OptionButton optGreaterThan 
         BackColor       =   &H00C0C0C0&
         Caption         =   "Greater Than"
         Height          =   255
         Left            =   120
         TabIndex        =   4
         Top             =   720
         Width           =   1455
      End
      Begin OptionButton optLessThan 
         BackColor       =   &H00C0C0C0&
         Caption         =   "Less Than"
         Height          =   255
         Left            =   120
         TabIndex        =   3
         Top             =   480
         Width           =   1335
      End
      Begin MaskEdBox meAmount 
      End
      Begin Label Label11 
         Alignment       =   2   'Center
         BackColor       =   &H00C0C0C0&
         Caption         =   "$"
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "MS Sans Serif"
         FontSize        =   9.75
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   255
         Left            =   120
         TabIndex        =   72
         Top             =   960
         Width           =   255
      End
   End
   Begin CommandButton cmdCreateReport 
      Caption         =   "&Create Report"
      Height          =   1095
      Left            =   6240
      TabIndex        =   30
      Top             =   3720
      Width           =   2055
   End
   Begin CommandButton cmdClear 
      Caption         =   "&Clear"
      Height          =   375
      Left            =   4440
      TabIndex        =   29
      Top             =   3720
      Width           =   1575
   End
   Begin Frame Frame3 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Choose Your Phone Lists"
      FontBold        =   -1  'True
      FontItalic      =   0   'False
      FontName        =   "Tahoma"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   1095
      Left            =   120
      TabIndex        =   40
      Top             =   3720
      Width           =   4215
      Begin FileListBox PhoneList 
         Height          =   615
         Left            =   240
         MultiSelect     =   2
         Pattern         =   "*.plw"
         TabIndex        =   28
         Top             =   240
         Width           =   3735
      End
   End
   Begin Frame Frame2 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Call Type"
      FontBold        =   -1  'True
      FontItalic      =   0   'False
      FontName        =   "Tahoma"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   2175
      Left            =   120
      TabIndex        =   39
      Top             =   1440
      Width           =   8295
      Begin CommandButton Command1 
         Caption         =   "&Clear Call Choices"
         Height          =   375
         Left            =   6360
         TabIndex        =   27
         Top             =   1680
         Width           =   1815
      End
      Begin CommandButton cmdSelectDialAround 
         Caption         =   "Select &Dial Around"
         Height          =   375
         Left            =   3360
         TabIndex        =   26
         Top             =   1680
         Width           =   1815
      End
      Begin CommandButton cmdSelectAll 
         Caption         =   "Select &All"
         Height          =   375
         Left            =   360
         TabIndex        =   25
         Top             =   1680
         Width           =   1815
      End
      Begin CheckBox Check1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "211"
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   255
         Index           =   3
         Left            =   480
         TabIndex        =   9
         Top             =   1080
         Width           =   615
      End
      Begin CheckBox Check1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "Incoming"
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         ForeColor       =   &H00404080&
         Height          =   255
         Index           =   18
         Left            =   6600
         TabIndex        =   24
         Top             =   1080
         Width           =   1095
      End
      Begin CheckBox Check1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "AOS 1"
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   255
         Index           =   7
         Left            =   2040
         TabIndex        =   13
         Top             =   840
         Width           =   855
      End
      Begin CheckBox Check1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "101-XXXX"
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   255
         Index           =   6
         Left            =   2040
         TabIndex        =   12
         Top             =   600
         Width           =   1095
      End
      Begin CheckBox Check1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "Send Fax "
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         ForeColor       =   &H00400000&
         Height          =   255
         Index           =   17
         Left            =   6600
         TabIndex        =   23
         Top             =   840
         Width           =   1095
      End
      Begin CheckBox Check1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "Recieve Fax"
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         ForeColor       =   &H00400000&
         Height          =   255
         Index           =   16
         Left            =   6600
         TabIndex        =   22
         Top             =   600
         Width           =   1335
      End
      Begin CheckBox Check1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "Data Port"
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         ForeColor       =   &H00400000&
         Height          =   255
         Index           =   15
         Left            =   6600
         TabIndex        =   21
         Top             =   360
         Width           =   1095
      End
      Begin CheckBox Check1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "Toll Free 800, 888, 877"
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   300
         Index           =   14
         Left            =   4080
         TabIndex        =   20
         Top             =   1320
         Width           =   2295
      End
      Begin CheckBox Check1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "OCC"
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   300
         Index           =   13
         Left            =   4080
         TabIndex        =   19
         Top             =   1080
         Width           =   735
      End
      Begin CheckBox Check1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "Instant Refund"
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   300
         Index           =   12
         Left            =   4080
         TabIndex        =   18
         Top             =   840
         Width           =   1695
      End
      Begin CheckBox Check1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "Dir Assistance"
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   255
         Index           =   11
         Left            =   4080
         TabIndex        =   17
         Top             =   600
         Width           =   1575
      End
      Begin CheckBox Check1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "Direct"
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   300
         Index           =   10
         Left            =   4080
         TabIndex        =   16
         Top             =   360
         Width           =   855
      End
      Begin CheckBox Check1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "AOS 3"
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   300
         Index           =   9
         Left            =   2040
         TabIndex        =   15
         Top             =   1320
         Width           =   855
      End
      Begin CheckBox Check1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "AOS 2"
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   255
         Index           =   8
         Left            =   2040
         TabIndex        =   14
         Top             =   1080
         Width           =   855
      End
      Begin CheckBox Check1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "950"
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   255
         Index           =   5
         Left            =   2040
         TabIndex        =   11
         Top             =   360
         Width           =   855
      End
      Begin CheckBox Check1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "611"
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   300
         Index           =   4
         Left            =   480
         TabIndex        =   10
         Top             =   1320
         Width           =   615
      End
      Begin CheckBox Check1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "0-"
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   300
         Index           =   2
         Left            =   480
         TabIndex        =   8
         Top             =   840
         Width           =   495
      End
      Begin CheckBox Check1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "0+"
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   300
         Index           =   1
         Left            =   480
         TabIndex        =   7
         Top             =   600
         Width           =   495
      End
      Begin CheckBox Check1 
         BackColor       =   &H00C0C0C0&
         Caption         =   "00"
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   300
         Index           =   0
         Left            =   480
         TabIndex        =   6
         Top             =   360
         Width           =   495
      End
   End
   Begin Frame Frame1 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Dates To Report"
      FontBold        =   -1  'True
      FontItalic      =   0   'False
      FontName        =   "Tahoma"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   1335
      Left            =   120
      TabIndex        =   38
      Top             =   120
      Width           =   2055
      Begin MaskEdBox meToDate 
      End
      Begin MaskEdBox meFromDate 
      End
      Begin Label Label6 
         Alignment       =   1   'Right Justify
         BackColor       =   &H8000000F&
         BackStyle       =   0   'Transparent
         Caption         =   "To"
         Height          =   255
         Left            =   180
         TabIndex        =   42
         Top             =   720
         Width           =   375
      End
      Begin Label Label5 
         Alignment       =   1   'Right Justify
         BackColor       =   &H8000000F&
         BackStyle       =   0   'Transparent
         Caption         =   "From"
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   255
         Left            =   60
         TabIndex        =   41
         Top             =   360
         Width           =   495
      End
   End
   Begin Frame fraDialAround 
      BackColor       =   &H00C0C0C0&
      Caption         =   "Dial Around"
      Enabled         =   0   'False
      FontBold        =   -1  'True
      FontItalic      =   0   'False
      FontName        =   "Tahoma"
      FontSize        =   8.25
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   1335
      Left            =   2280
      TabIndex        =   31
      Top             =   120
      Width           =   3975
      Begin MaskEdBox meCents 
      End
      Begin MaskEdBox meSeconds 
      End
      Begin Label Label4 
         BackColor       =   &H8000000F&
         BackStyle       =   0   'Transparent
         Caption         =   "cents per call."
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   255
         Left            =   2520
         TabIndex        =   37
         Top             =   840
         Width           =   1335
      End
      Begin Label Label3 
         BackColor       =   &H8000000F&
         BackStyle       =   0   'Transparent
         Caption         =   "Compensation is "
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   255
         Left            =   240
         TabIndex        =   36
         Top             =   840
         Width           =   1455
      End
      Begin Label Label2 
         BackColor       =   &H8000000F&
         BackStyle       =   0   'Transparent
         Caption         =   "seconds."
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   255
         Left            =   3000
         TabIndex        =   35
         Top             =   360
         Width           =   735
      End
      Begin Label Label1 
         BackColor       =   &H00C0C0C0&
         BackStyle       =   0   'Transparent
         Caption         =   "Call is assumed valid after "
         FontBold        =   -1  'True
         FontItalic      =   0   'False
         FontName        =   "Tahoma"
         FontSize        =   8.25
         FontStrikethru  =   0   'False
         FontUnderline   =   0   'False
         Height          =   255
         Left            =   240
         TabIndex        =   33
         Top             =   360
         Width           =   2295
      End
   End
   Begin TextBox Text1 
      DataField       =   "ANI"
      DataSource      =   "Data1"
      Height          =   285
      Index           =   0
      Left            =   240
      TabIndex        =   44
      Text            =   "Text1"
      Top             =   6120
      Visible         =   0   'False
      Width           =   1215
   End
End

