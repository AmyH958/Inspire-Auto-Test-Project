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

WebUI.setText(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_(15trunk)/input_(15trunk)_member_id'), 
    'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_(15trunk)/a_'), '流通')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_(15trunk)/a__1'), '辦證')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_(15trunk)/a__1_2'), '讀者證卡記錄維護')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_(15trunk)/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_(15trunk)/b_'), '查詢')

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_(15trunk)/h3_'), '查詢條件')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_(15trunk)/input__formSubmitSearch'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_(15trunk)/span_1'), '1')

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_(15trunk)/td_'), '編輯')

WebUI.verifyElementPresent(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_(15trunk)/img__Any_0_1'), 
    2)

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_(15trunk)/img__Any_0_1'))

WebUI.switchToWindowUrl('http://10.11.20.15/inspireapp/NavigareCarduri,crsArea.$DirectLink.sdirect?sp=l13067')

WebUI.switchToWindowTitle('Details')

WebUI.setText(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_Details/input__TextField_1'), '測試王_202402270250')

WebUI.scrollToElement(findTestObject('流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_Details/a_'), 2, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_Details/a_'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_Details/a_'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_Details/html_DetailsdjConfig  baseRelativePathinspi_19211e'))

WebUI.setText(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_Details/input__mobilePhone'), '')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_Details/a_'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_Details/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_Details/div_'), '存檔成功')

WebUI.closeWindowUrl('http://10.11.20.15/inspireapp/NavigareCarduri,crsArea.$DirectLink.sdirect?sp=l13067')

WebUI.switchToDefaultContent()

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/檢查讀者/讀者正卡記錄維護/編輯一筆讀者正卡紀錄正常編輯/Page_(15trunk)/a__1_2_3'))

WebUI.closeBrowser()

