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

WebUI.setText(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/a_'), '流通')

WebUI.click(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/a__1'), '借閱檯')

WebUI.click(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/a__1_2'), '讀者流通檯')

WebUI.click(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/td_'), '讀者證號/帳號/姓名')

WebUI.setText(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/input__cardNumberField'), '神通測試')

WebUI.click(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/input__submitSearch'))

WebUI.click(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/input__RadioGroup'))

WebUI.click(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.click(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/span_'))

WebUI.click(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/a_F2'))

WebUI.setText(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/input__itemNumberField'), 'AAA')

WebUI.verifyElementText(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/select_'), '條碼號\n書名')

WebUI.click(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/input__itemNumberField'))

WebUI.click(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/input__submitSearchItem'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/p_AAA ,'), '館藏條碼號 AAA 不存在,請重新輸入正確條碼號')

WebUI.takeFullPageScreenshotAsCheckpoint('還書失敗')

WebUI.click(findTestObject('Object Repository/流通/借閱台/還書失敗/Page_(15trunk)/a__1_2_3'))

WebUI.closeBrowser()

