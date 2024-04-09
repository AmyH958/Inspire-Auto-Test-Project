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

WebUI.navigateToUrl(GlobalVariable.Inspire_web)

WebUI.maximizeWindow()

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_(15trunk)/input_(15trunk)_member_id'))

WebUI.setText(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_(15trunk)/input_(15trunk)_member_id'), GlobalVariable.admin_name)

WebUI.setEncryptedText(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    GlobalVariable.admin_passwd)

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_(15trunk)/a_'), '流通')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_(15trunk)/a__1'), '辦證')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_(15trunk)/a__1_2'), '讀者記錄維護')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_(15trunk)/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_(15trunk)/a__1_2_3'), ' 新增 ')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_(15trunk)/a__1_2_3'))

WebUI.switchToWindowTitle('Details')

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_Details/a_'), '讀者資料維護')

WebUI.setText(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_Details/input__patronName'), username)

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_Details/input__patronName'))

WebUI.setText(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_Details/input__patronName'), username)

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_Details/input__patronName'))

WebUI.setText(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_Details/input__patronUsername'), UserIDNumber)

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_Details/a__1'), '檢查是否已辦證')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_Details/a__1'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_Details/b_202402260156'))

WebUI.setText(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_Details/input__nationalIdNumber'), UserIDNumber)

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_Details/td_MATLABPHP1'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_Details/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_Details/a__1_2_3'), '修改/存檔')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_Details/a__1_2_3'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_Details/a__1_2_3_4'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_Details/div_'), '存檔成功')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_Details/img__Any_8'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_Details/a__1_2_3_4_5'))

WebUI.switchToDefaultContent(FailureHandling.STOP_ON_FAILURE)

WebUI.enableSmartWait()

WebUI.setText(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_(15trunk)/input__field1'), UserIDNumber)

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_(15trunk)/input__formSubmitSearch'))

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_(15trunk)/select_'), '3', true)

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_(15trunk)/input__formSubmitSearch'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_(15trunk)/a_202402260156'), username)

WebUI.takeFullPageScreenshotAsCheckpoint('新增讀者成功')

WebUI.enhancedClick(findTestObject('Object Repository/流通/辦證/讀者記錄維護/正確新增讀者/Page_(15trunk)/a__1_2_3_4'))

WebUI.closeBrowser()

