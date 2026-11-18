VERSION 2.00
Begin Form frmNoModems 
   BackColor       =   &H00C0C0C0&
   Caption         =   "Modem Auto-Selection Message....."
   ClientHeight    =   3735
   ClientLeft      =   1095
   ClientTop       =   1485
   ClientWidth     =   6855
   Height          =   3735
   Icon            =   FRMNOMOD.FRX:0000
   Left            =   1095
   LinkTopic       =   "Form2"
   MaxButton       =   0   'False
   ScaleHeight     =   3735
   ScaleWidth      =   6855
   Top             =   1485
   Width           =   6855
   Begin Label Label3 
      BackStyle       =   0   'Transparent
      Caption         =   "     Until a modem is avaiable, this instance of Telelink for Windows will continue to function as normal, with the exception of being able to place or recieve calls."
      FontBold        =   -1  'True
      FontItalic      =   0   'False
      FontName        =   "MS Sans Serif"
      FontSize        =   9.75
      FontStrikethru  =   0   'False
      FontUnderline   =   0   'False
      Height          =   855
      Left            =   120
      TabIndex        =   2
      Top             =   2760
      Width           =   6615
   End
   Begin Label Label2 
      Caption         =   "     Before attempting to use your modem again, please make sure that all avaiable modems are not currently in use by other programs ( including other instances of Telelink ), check your system settings to make sure that your modems are configured corre"
   End
   Begin Label Label1 
      Caption         =   "7     Telelink's automatic modem selector has failed to find an available Hayes-compatable modem on this computer.  This error usually occurs when other instances of Telelink ( or other applications ) are currently using the modem(s) available on this sy"
   End
End

