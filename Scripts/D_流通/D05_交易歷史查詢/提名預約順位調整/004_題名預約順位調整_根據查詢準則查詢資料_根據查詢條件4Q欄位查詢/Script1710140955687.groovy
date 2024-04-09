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

WebUI.setText(findTestObject('Object Repository/流通/交易歷史查詢/題名預約順位調整/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_(15trunk)_member_id'), 
    'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/流通/交易歷史查詢/題名預約順位調整/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/題名預約順位調整/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/題名預約順位調整/根據查詢條件4Q欄位查詢/Page_(15trunk)/a_'), '流通')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/題名預約順位調整/根據查詢條件4Q欄位查詢/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/題名預約順位調整/根據查詢條件4Q欄位查詢/Page_(15trunk)/a__1'), '交易歷史查詢')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/題名預約順位調整/根據查詢條件4Q欄位查詢/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/題名預約順位調整/根據查詢條件4Q欄位查詢/Page_(15trunk)/a__1_2'), '題名預約順位調整')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/題名預約順位調整/根據查詢條件4Q欄位查詢/Page_(15trunk)/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/題名預約順位調整/根據查詢條件4Q欄位查詢/Page_(15trunk)/b_'), '查詢')

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/交易歷史查詢/題名預約順位調整/根據查詢條件4Q欄位查詢/Page_(15trunk)/select_'), '4', 
    true)

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/題名預約順位調整/根據查詢條件4Q欄位查詢/Page_(15trunk)/input__field1'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/題名預約順位調整/根據查詢條件4Q欄位查詢/Page_(15trunk)/body_dojo.registerModulePath(tapestry, insp_118d36'))

WebUI.setText(findTestObject('Object Repository/流通/交易歷史查詢/題名預約順位調整/根據查詢條件4Q欄位查詢/Page_(15trunk)/input__field1'), '李')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/題名預約順位調整/根據查詢條件4Q欄位查詢/Page_(15trunk)/h3_'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/題名預約順位調整/根據查詢條件4Q欄位查詢/Page_(15trunk)/input__formSubmitSearch'))

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/交易歷史查詢/題名預約順位調整/根據查詢條件4Q欄位查詢/Page_(15trunk)/select_andornot'), 
    '1', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/交易歷史查詢/題名預約順位調整/根據查詢條件4Q欄位查詢/Page_(15trunk)/select_andornot_1'), 
    '1', true)

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/題名預約順位調整/根據查詢條件4Q欄位查詢/Page_(15trunk)/div_()()()()                               _eac989'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/題名預約順位調整/根據查詢條件4Q欄位查詢/Page_(15trunk)/input__formSubmitSearch'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/題名預約順位調整/根據查詢條件4Q欄位查詢/Page_(15trunk)/a__1_2_3'))

WebUI.closeBrowser()

