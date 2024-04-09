import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://10.11.20.15/inspireapp/logon')

WebUI.maximizeWindow()

WebUI.enableSmartWait()

WebUI.setText(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.enableSmartWait()

WebUI.setEncryptedText(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a_'), '流通')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__1'), '盤點')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__1_2'), '盤點維護')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__1_2'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a_3'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__20240313'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/td_( starts with 0000000001) and   00000000_3442b7'))

WebUI.scrollToElement(findTestObject('流通/盤點/盤點維護/進行盤點/Page_(15trunk)/span_'), 2)

WebUI.verifyElementText(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/span_'), '刷入條碼號')

WebUI.setText(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/input__inputTxtField'), '\'0000000001')

WebUI.verifyElementText(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/input__btnUpload'), '')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/input__btnUpload'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/td_( starts with 0000000001) and   00000000_3442b7'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/td_( starts with 0000000001) and   00000000_3442b7'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/body_dojo.registerModulePath(tapestry, insp_118d36'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/input__inputTxtField'))

WebUI.setText(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/input__inputTxtField'), '\'0000000001')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/div_255'))

WebUI.setText(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/input__inputTxtField'), '00000000018')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/input__btnUpload'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/td_( starts with 0000000001) and   00000000_3442b7'))

WebUI.setText(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/input__inputTxtField'), '0000000018')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/input__btnUpload'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/td_0000000018'))

WebUI.setText(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/input__inputTxtField'), '0000000017')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/input__btnUpload'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__1_2_3'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__1_2_3_4'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__20240313'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__1_2_3_4_5'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__1_2_3_4'), FailureHandling.CONTINUE_ON_FAILURE)

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__20240313'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__1_2_3_4_5_6'))

WebUI.setText(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/input__TextField'), '0000000001')

WebUI.setText(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/input__TextField_0'), '0000000018')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__1_2_3_4_5_6_7'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/td_'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__1_2_3_4_5_6_7_8'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__1_2_3_4_5_6_7_8_9'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__1_2_3_4_5_6_7_8_9_10'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__1_2_3_4_5_6_7_8_9_10_11'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a_ViewReport'))

WebUI.switchToWindowTitle('神資圖書館(15trunk機)')

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__1_2_3_4_5_6_7_8_9_10_11_12'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__1_2_3_4_5_6_7_8'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__1_2_3_4_5_6_7_8_9_10_11_12_13'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/span__1'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__1_2_3_4'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/盤點/盤點維護/進行盤點/Page_(15trunk)/a__1_2_3_4_5_6_7_8_9_10_11_12_13_14'))

WebUI.closeBrowser()

