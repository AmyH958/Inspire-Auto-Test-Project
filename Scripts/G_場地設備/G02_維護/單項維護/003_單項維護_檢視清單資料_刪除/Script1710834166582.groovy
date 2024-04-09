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

WebUI.setText(findTestObject('Object Repository/場地設備/維護/單項維護/檢視清單資料刪除/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/場地設備/維護/單項維護/檢視清單資料刪除/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/檢視清單資料刪除/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/維護/單項維護/檢視清單資料刪除/Page_(15trunk)/a_'), '場地設備')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/檢視清單資料刪除/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/維護/單項維護/檢視清單資料刪除/Page_(15trunk)/a__1'), '維護')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/檢視清單資料刪除/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/維護/單項維護/檢視清單資料刪除/Page_(15trunk)/a__1_2'), '單項維護')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/檢視清單資料刪除/Page_(15trunk)/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/維護/單項維護/檢視清單資料刪除/Page_(15trunk)/b_'), '查詢')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/檢視清單資料刪除/Page_(15trunk)/b_'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/檢視清單資料刪除/Page_(15trunk)/input__formSubmitSearch'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/檢視清單資料刪除/Page_(15trunk)/img__Any_0_1'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/維護/單項維護/檢視清單資料刪除/Page_(15trunk)/div_'), '使用者權限不足，不允許刪除')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/檢視清單資料刪除/Page_(15trunk)/span_'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/檢視清單資料刪除/Page_(15trunk)/input__rootCheck'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/檢視清單資料刪除/Page_(15trunk)/a__1_2_3'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/檢視清單資料刪除/Page_(15trunk)/span_'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/檢視清單資料刪除/Page_(15trunk)/td'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/檢視清單資料刪除/Page_(15trunk)/input__rootCheck'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/維護/單項維護/檢視清單資料刪除/Page_(15trunk)/a__1_2_3_4'))

WebUI.closeBrowser()

