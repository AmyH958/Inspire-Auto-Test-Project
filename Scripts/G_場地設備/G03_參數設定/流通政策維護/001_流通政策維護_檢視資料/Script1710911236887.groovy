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

WebUI.setText(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a_'), '場地設備')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a__1'), '參數設定')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a__1'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a__1_2'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a__1_2_3'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a__1_2_3_4'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a__1_2_3'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a__1_2_3_4_5'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a__1_2_3_4_5_6'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a_()'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a__1_2_3_4_5_6_7'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a__1_2_3_4_5_6_7_8'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a__1_2_3_4_5_6_7'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a__1_2_3_4_5_6_7_8_9'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a_()_1'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a__1_2_3_4_5_6_7_8_9_10'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a__1_2_3_4_5_6_7_8_9_10_11'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a__1_2_3_4_5_6_7_8_9_10_11_12'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a__1_2_3_4_5_6_7_8_9_10_11_12_13'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a__1_2_3_4_5_6_7_8_9_10_11_12_13_14'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/參數設定/檢視資料查詢/Page_(15trunk)/a__1_2_3_4_5_6_7_8_9_10_11_12_13_14_15'))

WebUI.closeBrowser()

