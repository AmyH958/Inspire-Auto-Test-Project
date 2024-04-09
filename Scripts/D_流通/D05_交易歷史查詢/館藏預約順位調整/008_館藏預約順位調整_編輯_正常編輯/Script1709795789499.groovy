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

WebUI.setText(findTestObject('Object Repository/流通/交易歷史查詢/館藏預約順位調整/正常編輯/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/流通/交易歷史查詢/館藏預約順位調整/正常編輯/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/館藏預約順位調整/正常編輯/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/館藏預約順位調整/正常編輯/Page_(15trunk)/a_'), '流通')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/館藏預約順位調整/正常編輯/Page_(15trunk)/a__1'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/館藏預約順位調整/正常編輯/Page_(15trunk)/a_'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/館藏預約順位調整/正常編輯/Page_(15trunk)/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/館藏預約順位調整/正常編輯/Page_(15trunk)/a__1_2_3'), '館藏預約順位調整')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/館藏預約順位調整/正常編輯/Page_(15trunk)/a__1_2_3'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/館藏預約順位調整/正常編輯/Page_(15trunk)/b_'), '查詢')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/館藏預約順位調整/正常編輯/Page_(15trunk)/input__formSubmitSearch'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/交易歷史查詢/館藏預約順位調整/正常編輯/Page_(15trunk)/span_1'), '1')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/館藏預約順位調整/正常編輯/Page_(15trunk)/a_0000000034'))

WebUI.switchToWindowTitle('iNspire')

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/館藏預約順位調整/正常編輯/Page_iNspire/a_'))

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/交易歷史查詢/館藏預約順位調整/正常編輯/Page_iNspire/select_000102030405060708091011121314151617_d567da'), 
    '14', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/交易歷史查詢/館藏預約順位調整/正常編輯/Page_iNspire/select_000102030405060708091011121314151617_d567da_1'), 
    '22', true)

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/館藏預約順位調整/正常編輯/Page_iNspire/input__Submit'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/館藏預約順位調整/正常編輯/Page_iNspire/fieldset_'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/館藏預約順位調整/正常編輯/Page_iNspire/a__1'))

WebUI.switchToDefaultContent(FailureHandling.STOP_ON_FAILURE)

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/館藏預約順位調整/正常編輯/Page_(15trunk)/a_0000000034'))

WebUI.switchToDefaultContent(FailureHandling.STOP_ON_FAILURE)

WebUI.enhancedClick(findTestObject('Object Repository/流通/交易歷史查詢/館藏預約順位調整/正常編輯/Page_(15trunk)/a__1_2_3_4'))

WebUI.closeBrowser()

