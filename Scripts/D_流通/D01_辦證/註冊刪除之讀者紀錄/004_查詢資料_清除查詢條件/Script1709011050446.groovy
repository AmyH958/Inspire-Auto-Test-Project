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

WebUI.setText(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/a_'), '流通')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/a__1'), '辦證')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/a__1_2'), '註記刪除之讀者記錄')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/a__1_2'))

WebUI.setText(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/input__field1'), '李')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/p_andornot  ()()()()'))

WebUI.setText(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/input__field2'), 'A')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/div_'))

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/select_'), '6', true)

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/body_dojo.registerModulePath(tapestry, insp_118d36'))

WebUI.setText(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/input_A100000001_field3'), '0')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/input__formSubmitSearch'))

WebUI.setText(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/input_A100000001_field3'), '')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/input__formSubmitSearch'))

WebUI.setText(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/input__field2'), '')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/input__formSubmitSearch'))

WebUI.setText(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/input__field1'), '')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/input__formSubmitSearch'))

WebUI.setText(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/input__field1'), '楊')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/input__formSubmitSearch'))

WebUI.takeFullPageScreenshot()

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/input__resetbutton'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/input__field1'), '')

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/input__field2'), '')

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/input_A100000001_DatePicker_1'), 
    '')

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/input__DatePicker_2'), '')

WebUI.takeFullPageScreenshotAsCheckpoint('回到初始畫面')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/註記刪除之讀者記錄/清除查詢資料/Page_(15trunk)/a__1_2_3'))

WebUI.closeBrowser()

