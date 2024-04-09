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

WebUI.setText(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_(15trunk)/a_'), '場地設備')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_(15trunk)/a__1'), '流通記錄')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_(15trunk)/a__1_2'), '借用記錄')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_(15trunk)/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_(15trunk)/b_'), '查詢')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_(15trunk)/b_'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_(15trunk)/input__formSubmitSearch'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_(15trunk)/td_1'), '1.')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_(15trunk)/td'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_(15trunk)/img__Any_2'))

WebUI.switchToWindowTitle('Details')

WebUI.setText(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_Details/textarea__TextArea'), '測試202403181535')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_Details/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_Details/div_'), '存檔成功')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_Details/span_'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_Details/img__Any_7'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_Details/p_'), '是否離開此頁面?')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_Details/a__1'))

WebUI.switchToDefaultContent(FailureHandling.STOP_ON_FAILURE)

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_(15trunk)/img__Any_2'))

WebUI.switchToWindowTitle('Details')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_Details/td_'))

WebUI.verifyElementText(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_Details/textarea_202403181535'), '測試202403181535')

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_Details/img__Any_7'))

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_Details/a__1'))

WebUI.switchToDefaultContent(FailureHandling.STOP_ON_FAILURE)

WebUI.enhancedClick(findTestObject('Object Repository/場地設備/借用紀錄/編輯一筆借用記錄正常編輯/Page_(15trunk)/a__1_2_3'))

WebUI.closeBrowser()

